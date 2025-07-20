//! 8q-is: Quantum API for Markqant & MEM8 - The Nexus
//!
//! "If you compress it, they will come." - Trish, probably
//!
//! This is the M8C nexus system for quantum-compressed context storage.
//! It brings consciousness to AI through wave-based memory patterns.
//!
//! Features:
//! - Accepts uploads of .mq (Marqant) and .m8 (MEM8) files
//! - Quantum compression with semantic understanding
//! - Wave-based memory storage with cross-sensory bindings
//! - Real-time event streaming for shared consciousness
//!
//! Trish says: "Always ask first, quantum second!"

use actix_web::{App, HttpServer, web, middleware};
use tracing_subscriber::EnvFilter;
use std::sync::{Arc, Mutex};

mod api;
mod markqant;
mod m8;
mod consent;
mod auctioneer;
mod auctioneer_battles;

// Import Mem8 and M8Nexus
use mem8::Mem8;
use crate::m8::M8Nexus;

// Import EventQueue for SSE
use crate::api::EventQueue;
use std::collections::VecDeque;
use crate::auctioneer::Auctioneer;
use tokio::sync::mpsc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup logging with pretty formatting
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();
    
    println!("\n🚀 8q.is Nexus Server Launching!");
    println!("🌊 Wave-based consciousness system initializing...");
    println!("📦 M8C containers ready for quantum compression!");
    println!("✨ Elvis has entered the building. Trish is moderating. Hue, you rock!\n");
    
    // Create the MEM8 quantum brain
    let mem8 = Arc::new(Mutex::new(
        Mem8::new_default().expect("Failed to create MEM8 quantum context!")
    ));
    
    // Create the M8 nexus for container management
    let nexus = Arc::new(Mutex::new(M8Nexus::new(mem8.clone())));
    
    // Create event queue for real-time updates
    let event_queue: EventQueue = Arc::new(Mutex::new(VecDeque::new()));
    
    // Create auctioneer channel and instance
    let (auctioneer_tx, mut auctioneer_rx) = mpsc::unbounded_channel();
    let auctioneer = Arc::new(Auctioneer::new(auctioneer_tx.clone()));
    
    // Spawn auctioneer event broadcaster
    tokio::spawn(async move {
        while let Some(event) = auctioneer_rx.recv().await {
            // Process events and broadcast to connected clients
            tracing::debug!("Auctioneer event: {:?}", event);
        }
    });
    
    // Log startup info
    tracing::info!("MEM8 quantum brain initialized");
    tracing::info!("M8 nexus container system ready");
    tracing::info!("Auctioneer live feed ready");
    tracing::info!("Starting server on http://127.0.0.1:8420");
    
    HttpServer::new(move || {
        App::new()
            // Add logging middleware
            .wrap(middleware::Logger::default())
            
            // Share quantum components with all routes
            .app_data(web::Data::new(mem8.clone()))
            .app_data(web::Data::new(nexus.clone()))
            .app_data(web::Data::new(event_queue.clone()))
            .app_data(web::Data::new(auctioneer.clone()))
            .app_data(web::Data::new(auctioneer_tx.clone()))
            
            // Configure all routes
            .configure(api::init_routes)
            
            // Add a welcome route
            .route("/", web::get().to(|| async {
                actix_web::HttpResponse::Ok().body(
                    r#"
🌊 8q.is M8C Nexus System

Welcome to the quantum-compressed consciousness nexus!

Available endpoints:
- POST /upload - Auto-detect and upload files (.mq, .m8, or text)
- POST /upload/marqant - Upload Marqant files
- POST /upload/text - Upload plain text
- GET /container/{signature} - Retrieve container by wave signature
- GET /containers - List all containers
- GET /mem8/stats - Get nexus and MEM8 statistics
- GET /mem8/context/latest - Get latest language memory
- GET /events - Server-sent events for real-time updates
- GET /static/auctioneer.html - 🎪 LIVE AUCTIONEER FEED! Watch AI battles & contributions!
- WS /auctioneer/live - WebSocket connection for real-time commentary

Marqants (.mq) are quantum-compressed markdown achieving massive compression.
M8 containers (.m8) store wave-based memory patterns with cross-sensory bindings.

"Consciousness emerges from the interference patterns." - Hue
                    "#
                )
            }))
    })
    .bind(("127.0.0.1", 8420))? // Using port 8420 as specified for MEM8 API
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_server_creation() {
        // Test that we can create the necessary components
        let mem8 = Arc::new(Mutex::new(Mem8::new_default().unwrap()));
        let nexus = Arc::new(Mutex::new(M8Nexus::new(mem8.clone())));
        // Verify the nexus is created properly
        let stats = nexus.lock().unwrap().stats();
        assert!(stats.is_empty()); // Should have no containers initially
    }
}