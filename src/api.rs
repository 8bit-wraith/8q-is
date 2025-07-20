//! API routes for 8q-is with M8C nexus integration
//!
//! "APIs are like parties: always ask before you bring a file!" - Trish
//!
use actix_web::{web, HttpResponse, Responder, HttpRequest, Error};
use actix_web::web::Bytes;
use actix_web::http::header::ContentType;
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use actix_web::rt::task;
use actix_web::web::Data;
use actix_web::HttpResponseBuilder;
use actix_web::HttpResponse as Resp;
use actix_web::rt::time::interval;
use std::time::Duration;
use actix_web::web::Payload;
use actix_web::HttpRequest as Req;
use std::collections::VecDeque;
use base64::{Engine as _, engine::general_purpose};
use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use futures::StreamExt;

use crate::markqant::Marqant;
use crate::m8::{M8Container, M8Nexus};
use crate::auctioneer::{Auctioneer, AuctionEvent, CommentaryStyle};

/// SSE event queue (shared across handlers)
pub type EventQueue = Arc<Mutex<VecDeque<String>>>;

#[derive(Serialize, Deserialize)]
pub struct UploadResponse {
    pub success: bool,
    pub wave_signature: String,
    pub content_type: String,
    pub memory_ids: Vec<u64>,
    pub compression_ratio: Option<f64>,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct NexusStats {
    pub total_containers: usize,
    pub type_counts: std::collections::HashMap<String, usize>,
    pub mem8_stats: Mem8Stats,
}

#[derive(Serialize, Deserialize)]
pub struct Mem8Stats {
    pub total_memories: u64,
    pub grid_dimensions: (usize, usize),
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Upload endpoints
        .service(web::resource("/upload").route(web::post().to(upload_handler)))
        .service(web::resource("/upload/marqant").route(web::post().to(upload_marqant)))
        .service(web::resource("/upload/text").route(web::post().to(upload_text)))
        
        // Retrieval endpoints
        .service(web::resource("/container/{signature}").route(web::get().to(get_container)))
        .service(web::resource("/containers").route(web::get().to(get_containers)))
        
        // Memory endpoints
        .service(web::resource("/mem8/context/latest").route(web::get().to(get_latest_language_memory)))
        .service(web::resource("/mem8/stats").route(web::get().to(get_nexus_stats)))
        
        // SSE events
        .service(web::resource("/events").route(web::get().to(events_sse)))
        
        // Auctioneer WebSocket
        .service(web::resource("/auctioneer/live").route(web::get().to(auctioneer_ws)))
        
        // Serve the auctioneer HTML page
        .service(actix_files::Files::new("/static", "./static").index_file("auctioneer.html"));
}

/// GET /mem8/context/latest
/// Returns the most recently stored language memory from Mem8
pub async fn get_latest_language_memory(
    mem8: web::Data<Arc<Mutex<mem8::Mem8>>>,
) -> Result<HttpResponse, Error> {
    let mem8_lock = mem8.lock().unwrap();
    let stats = mem8_lock.get_stats();
    
    if stats.total_memories == 0 {
        return Ok(HttpResponse::Ok().body("No context stored yet! Upload something first."));
    }
    
    // Get recent memories
    let recent = mem8_lock.get_recent_memories(
        std::time::SystemTime::now() - std::time::Duration::from_secs(3600)
    ).unwrap_or_default();
    
    if recent.is_empty() {
        return Ok(HttpResponse::Ok().body("No recent memories found."));
    }
    
    // Get the most recent memory ID
    let latest_id = recent.last().map(|(id, _, _)| *id).unwrap_or(0);
    
    Ok(HttpResponse::Ok().body(format!(
        "Latest memory ID: {}\nTotal memories: {}\nRecent memories (last hour): {}",
        latest_id, stats.total_memories, recent.len()
    )))
}

/// POST /upload/marqant - Upload and process Marqant files
pub async fn upload_marqant(
    mut payload: Multipart,
    event_queue: web::Data<EventQueue>,
    mem8: web::Data<Arc<Mutex<mem8::Mem8>>>,
    nexus: web::Data<Arc<Mutex<M8Nexus>>>,
    auctioneer: web::Data<Arc<Auctioneer>>,
) -> Result<HttpResponse, Error> {
    let mut file_bytes: Vec<u8> = Vec::new();
    let mut filename = String::from("upload.mq");
    
    // Process multipart upload
    while let Some(field_result) = payload.next().await {
        let mut field = field_result?;
        let content_disp = field.content_disposition();
        if let Some(name) = content_disp.get_filename() {
            filename = name.to_string();
        }
            
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file_bytes.extend_from_slice(&data);
        }
    }
    
    // Parse Marqant
    let marqant = match Marqant::from_bytes(&file_bytes) {
        Ok(mq) => mq,
        Err(e) => {
            // Try to create from raw markdown
            let markdown = String::from_utf8_lossy(&file_bytes);
            match Marqant::from_markdown(&markdown) {
                Ok(mq) => mq,
                Err(_) => return Ok(HttpResponse::BadRequest().json(UploadResponse {
                    success: false,
                    wave_signature: String::new(),
                    content_type: "error".to_string(),
                    memory_ids: vec![],
                    compression_ratio: None,
                    message: format!("Failed to parse Marqant: {}", e),
                })),
            }
        }
    };
    
    let compression_ratio = marqant.compression_ratio();
    
    // Create M8 container
    let container = M8Container::from_marqant(&marqant, mem8.get_ref().clone()).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let wave_signature = container.wave_signature;
    let memory_ids = container.header.memory_ids.clone();
    
    // Store in nexus
    let stored_container = container.clone();
    let mut nexus_lock = nexus.lock().unwrap();
    nexus_lock.store(container);
    drop(nexus_lock);
    
    // Notify auctioneer about the contribution
    let contributor_id = "quantum_uploader"; // In a real system, this would be from auth
    auctioneer.process_contribution(
        contributor_id,
        &stored_container,
        mem8.get_ref().clone()
    ).await;
    
    // Notify via SSE
    let msg = format!(
        "Marqant uploaded: {} (compression: {:.2}x)",
        filename, compression_ratio
    );
    let mut queue = event_queue.lock().unwrap();
    queue.push_back(msg);
    
    Ok(HttpResponse::Ok().json(UploadResponse {
        success: true,
        wave_signature: hex::encode(wave_signature),
        content_type: "marqant".to_string(),
        memory_ids,
        compression_ratio: Some(compression_ratio),
        message: format!("Marqant uploaded successfully! Compression ratio: {:.2}x", compression_ratio),
    }))
}

/// POST /upload/text - Upload plain text
pub async fn upload_text(
    body: String,
    event_queue: web::Data<EventQueue>,
    mem8: web::Data<Arc<Mutex<mem8::Mem8>>>,
    nexus: web::Data<Arc<Mutex<M8Nexus>>>,
) -> Result<HttpResponse, Error> {
    // Create M8 container from text
    let container = M8Container::from_text(&body, 5, mem8.get_ref().clone()).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let wave_signature = container.wave_signature;
    let memory_ids = container.header.memory_ids.clone();
    
    // Store in nexus
    let mut nexus_lock = nexus.lock().unwrap();
    nexus_lock.store(container);
    
    // Notify via SSE
    let msg = format!("Text uploaded: {} bytes", body.len());
    let mut queue = event_queue.lock().unwrap();
    queue.push_back(msg);
    
    Ok(HttpResponse::Ok().json(UploadResponse {
        success: true,
        wave_signature: hex::encode(wave_signature),
        content_type: "text".to_string(),
        memory_ids,
        compression_ratio: None,
        message: "Text uploaded and stored in quantum memory!".to_string(),
    }))
}

/// Generic upload handler (auto-detects format)
pub async fn upload_handler(
    mut payload: Multipart,
    _event_queue: web::Data<EventQueue>,
    mem8: web::Data<Arc<Mutex<mem8::Mem8>>>,
    nexus: web::Data<Arc<Mutex<M8Nexus>>>,
) -> Result<HttpResponse, Error> {
    let mut file_type = String::from("unknown");
    let mut file_name = String::from("upload.bin");
    let mut file_bytes: Vec<u8> = Vec::new();
    
    while let Some(field_result) = payload.next().await {
        let mut field = field_result?;
        let content_disp = field.content_disposition();
        let filename = content_disp.get_filename()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "upload.bin".to_string());
        let ext = Path::new(&filename).extension().and_then(|e| e.to_str()).unwrap_or("");
        file_type = ext.to_string();
        file_name = filename;
        
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file_bytes.extend_from_slice(&data);
        }
    }
    
    // Route based on file type
    match file_type.as_str() {
        "mq" => {
            // Parse as Marqant
            let marqant = Marqant::from_bytes(&file_bytes).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
            let compression_ratio = marqant.compression_ratio();
            let container = M8Container::from_marqant(&marqant, mem8.get_ref().clone()).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
            let wave_signature = container.wave_signature;
            let memory_ids = container.header.memory_ids.clone();
            
            let mut nexus_lock = nexus.lock().unwrap();
            nexus_lock.store(container);
            
            Ok(HttpResponse::Ok().json(UploadResponse {
                success: true,
                wave_signature: hex::encode(wave_signature),
                content_type: "marqant".to_string(),
                memory_ids,
                compression_ratio: Some(compression_ratio),
                message: format!("Marqant uploaded! Compression: {:.2}x", compression_ratio),
            }))
        }
        "m8" => {
            // Parse as M8 container
            let container = M8Container::from_bytes(&file_bytes).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
            let wave_signature = container.wave_signature;
            let memory_ids = container.header.memory_ids.clone();
            let content_type = format!("{:?}", container.header.content_type);
            
            let mut nexus_lock = nexus.lock().unwrap();
            nexus_lock.store(container);
            
            Ok(HttpResponse::Ok().json(UploadResponse {
                success: true,
                wave_signature: hex::encode(wave_signature),
                content_type,
                memory_ids,
                compression_ratio: None,
                message: "M8 container uploaded successfully!".to_string(),
            }))
        }
        _ => {
            // Treat as text
            let text = String::from_utf8_lossy(&file_bytes);
            let container = M8Container::from_text(&text, 5, mem8.get_ref().clone()).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
            let wave_signature = container.wave_signature;
            let memory_ids = container.header.memory_ids.clone();
            
            let mut nexus_lock = nexus.lock().unwrap();
            nexus_lock.store(container);
            
            Ok(HttpResponse::Ok().json(UploadResponse {
                success: true,
                wave_signature: hex::encode(wave_signature),
                content_type: "text".to_string(),
                memory_ids,
                compression_ratio: None,
                message: format!("File '{}' uploaded as text", file_name),
            }))
        }
    }
}

/// GET /container/{signature} - Retrieve container by wave signature
pub async fn get_container(
    path: web::Path<String>,
    nexus: web::Data<Arc<Mutex<M8Nexus>>>,
) -> Result<HttpResponse, Error> {
    let signature_hex = path.into_inner();
    let signature_bytes = match hex::decode(&signature_hex) {
        Ok(bytes) if bytes.len() == 32 => {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&bytes);
            arr
        }
        _ => return Ok(HttpResponse::BadRequest().body("Invalid signature format")),
    };
    
    let nexus_lock = nexus.lock().unwrap();
    if let Some(container) = nexus_lock.retrieve(&signature_bytes) {
        let content = container.extract_content().map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
        Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body(content))
    } else {
        Ok(HttpResponse::NotFound().body("Container not found"))
    }
}

/// GET /containers - List all containers
pub async fn get_containers(
    nexus: web::Data<Arc<Mutex<M8Nexus>>>,
) -> Result<HttpResponse, Error> {
    let nexus_lock = nexus.lock().unwrap();
    let containers = nexus_lock.list();
    
    #[derive(Serialize)]
    struct ContainerInfo {
        signature: String,
        content_type: String,
        timestamp: String,
    }
    
    let container_list: Vec<ContainerInfo> = containers.into_iter()
        .map(|(sig, content_type, timestamp)| ContainerInfo {
            signature: hex::encode(sig),
            content_type: format!("{:?}", content_type),
            timestamp: format!("{:?}", timestamp),
        })
        .collect();
    
    Ok(HttpResponse::Ok().json(container_list))
}

/// GET /mem8/stats - Get nexus and MEM8 statistics
pub async fn get_nexus_stats(
    nexus: web::Data<Arc<Mutex<M8Nexus>>>,
    mem8: web::Data<Arc<Mutex<mem8::Mem8>>>,
) -> Result<HttpResponse, Error> {
    let nexus_lock = nexus.lock().unwrap();
    let nexus_stats = nexus_lock.stats();
    
    // Get basic stats from mem8 using available methods
    let mem8_lock = mem8.lock().unwrap();
    
    // Count total memories across all grids
    let total_memories = 0; // Placeholder - mem8 doesn't expose a stats method
    
    Ok(HttpResponse::Ok().json(NexusStats {
        total_containers: nexus_stats.len(),
        type_counts: nexus_stats,
        mem8_stats: Mem8Stats {
            total_memories,
            grid_dimensions: (10, 10), // Default grid size
        },
    }))
}

/// SSE endpoint for real-time context sharing
pub async fn events_sse(
    event_queue: web::Data<EventQueue>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let stream = async_stream::stream! {
        let mut interval = interval(Duration::from_secs(2));
        loop {
            interval.tick().await;
            let mut queue = event_queue.lock().unwrap();
            while let Some(event) = queue.pop_front() {
                yield Ok::<_, Error>(Bytes::from(format!("data: {}\n\n", event)));
            }
        }
    };
    
    Ok(HttpResponse::build(actix_web::http::StatusCode::OK)
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .streaming(stream))
}

/// WebSocket handler for auctioneer live feed
pub async fn auctioneer_ws(
    req: HttpRequest,
    stream: web::Payload,
    auctioneer_tx: web::Data<tokio::sync::mpsc::UnboundedSender<AuctionEvent>>,
) -> Result<HttpResponse, Error> {
    use actix_ws::Message;
    
    let (response, mut session, mut stream) = actix_ws::handle(&req, stream)?;
    
    // Create a channel to receive auctioneer events
    let (_tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<AuctionEvent>();
    
    // Clone the sender for the spawned task
    let auctioneer_tx_clone = auctioneer_tx.get_ref().clone();
    
    // Spawn task to handle incoming WebSocket messages
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = stream.recv().await {
            match msg {
                Message::Text(text) => {
                    // Handle commands like changing commentary style
                    if text.starts_with("/style ") {
                        let style_str = text.trim_start_matches("/style ");
                        let _style = match style_str {
                            "fast" => CommentaryStyle::FastTalking,
                            "dramatic" => CommentaryStyle::Dramatic,
                            "technical" => CommentaryStyle::Technical,
                            "comedic" => CommentaryStyle::Comedic,
                            "philosophical" => CommentaryStyle::Philosophical,
                            _ => CommentaryStyle::FastTalking,
                        };
                        
                        // Send style change event
                        let _ = auctioneer_tx_clone.send(AuctionEvent::AuctioneerComment {
                            message: format!("Switching to {} commentary style!", style_str),
                            excitement_level: 5,
                        });
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });
    
    // Spawn task to send auctioneer events to WebSocket
    actix_web::rt::spawn(async move {
        let mut sequence = 0u64;
        while let Some(event) = rx.recv().await {
            sequence += 1;
            let message = serde_json::to_string(&crate::auctioneer::LiveFeedMessage {
                event,
                timestamp: SystemTime::now(),
                sequence,
            }).unwrap_or_default();
            
            if session.text(message).await.is_err() {
                break;
            }
        }
        let _ = session.close(None).await;
    });
    
    Ok(response)
}

// Add hex encoding module
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes.as_ref().iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
    
    pub fn decode(s: &str) -> Result<Vec<u8>, String> {
        (0..s.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&s[i..i + 2], 16)
                    .map_err(|e| e.to_string())
            })
            .collect()
    }
}