//! MEM8 container format (.m8) for quantum context storage
//!
//! "Binary is beautiful if you add enough quantum!" - Trish
//!
//! M8 containers store wave-based memory patterns with cross-sensory bindings

use mem8::{Mem8, WavePattern, EmotionalContext, Result};
use crate::markqant::Marqant;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Magic bytes for .m8 files
pub const M8_MAGIC: &[u8] = b"M8C1"; // MEM8 Container v1

/// M8 container types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum M8ContentType {
    Marqant,      // Quantum-compressed markdown
    WavePattern,  // Raw wave pattern
    Language,     // Text/language memory
    Visual,       // Visual memory
    Audio,        // Audio memory
    Compound,     // Multiple bound memories
}

/// M8 container header
#[derive(Debug, Serialize, Deserialize)]
pub struct M8Header {
    pub version: u8,
    pub content_type: M8ContentType,
    pub timestamp: std::time::SystemTime,
    pub memory_ids: Vec<u64>, // Associated memory IDs in MEM8
    pub emotional_context: [u8; 3], // 3-byte emotional state
    pub metadata: HashMap<String, String>,
}

/// M8 container - nexus between files and wave memory
#[derive(Debug)]
pub struct M8Container {
    pub header: M8Header,
    pub data: Vec<u8>,
    pub wave_signature: [u8; 32],
}

impl M8Container {
    /// Create M8 container from Marqant
    pub fn from_marqant(marqant: &Marqant, mem8: Arc<Mutex<Mem8>>) -> Result<Self> {
        let marqant_bytes = marqant.to_bytes()?;
        
        // Store in MEM8 as language memory with semantic understanding
        let markdown = marqant.to_markdown()?;
        let mut mem8_lock = mem8.lock().unwrap();
        let memory_id = mem8_lock.store_language(&markdown, 7)?; // High importance
        drop(mem8_lock);
        
        // Create emotional context (neutral for now)
        let emotional_context = [128u8, 128u8, 128u8]; // Neutral valence, arousal, dominance
        
        let header = M8Header {
            version: 1,
            content_type: M8ContentType::Marqant,
            timestamp: std::time::SystemTime::now(),
            memory_ids: vec![memory_id],
            emotional_context,
            metadata: HashMap::from([
                ("source".to_string(), "marqant".to_string()),
                ("compression_ratio".to_string(), format!("{:.2}", marqant.compression_ratio())),
            ]),
        };
        
        let wave_signature = Self::calculate_signature(&marqant_bytes);
        
        Ok(M8Container {
            header,
            data: marqant_bytes,
            wave_signature,
        })
    }
    
    /// Create M8 container from raw text
    pub fn from_text(text: &str, importance: u8, mem8: Arc<Mutex<Mem8>>) -> Result<Self> {
        // Store in MEM8
        let mut mem8_lock = mem8.lock().unwrap();
        let memory_id = mem8_lock.store_language(text, importance)?;
        drop(mem8_lock);
        
        let emotional_context = [128u8, 128u8, 128u8]; // Neutral
        
        let header = M8Header {
            version: 1,
            content_type: M8ContentType::Language,
            timestamp: std::time::SystemTime::now(),
            memory_ids: vec![memory_id],
            emotional_context,
            metadata: HashMap::from([
                ("source".to_string(), "text".to_string()),
                ("length".to_string(), text.len().to_string()),
            ]),
        };
        
        let data = text.as_bytes().to_vec();
        let wave_signature = Self::calculate_signature(&data);
        
        Ok(M8Container {
            header,
            data,
            wave_signature,
        })
    }
    
    /// Create compound M8 container binding multiple memories
    pub fn from_compound(
        memory_ids: Vec<u64>, 
        emotional_context: EmotionalContext,
        mem8: Arc<Mutex<Mem8>>
    ) -> Result<Self> {
        // Get wave patterns for all memories
        let mem8_lock = mem8.lock().unwrap();
        let mut wave_data = Vec::new();
        
        for &id in &memory_ids {
            if let Ok(wave) = mem8_lock.get_wave_pattern(id) {
                // Serialize wave pattern (simplified for now)
                wave_data.extend_from_slice(&id.to_le_bytes());
                wave_data.extend_from_slice(&(wave.amplitude.len() as u32).to_le_bytes());
                for &amp in &wave.amplitude {
                    wave_data.extend_from_slice(&amp.to_le_bytes());
                }
            }
        }
        drop(mem8_lock);
        
        let header = M8Header {
            version: 1,
            content_type: M8ContentType::Compound,
            timestamp: std::time::SystemTime::now(),
            memory_ids,
            emotional_context: emotional_context.to_bytes(),
            metadata: HashMap::from([
                ("source".to_string(), "compound".to_string()),
                ("memory_count".to_string(), memory_ids.len().to_string()),
            ]),
        };
        
        let wave_signature = Self::calculate_signature(&wave_data);
        
        Ok(M8Container {
            header,
            data: wave_data,
            wave_signature,
        })
    }
    
    /// Serialize to .m8 format
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        
        // Magic bytes
        output.extend_from_slice(M8_MAGIC);
        
        // Header
        let header_bytes = bincode::serialize(&self.header)?;
        output.extend_from_slice(&(header_bytes.len() as u32).to_le_bytes());
        output.extend_from_slice(&header_bytes);
        
        // Wave signature
        output.extend_from_slice(&self.wave_signature);
        
        // Data
        output.extend_from_slice(&(self.data.len() as u64).to_le_bytes());
        output.extend_from_slice(&self.data);
        
        Ok(output)
    }
    
    /// Deserialize from .m8 format
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < 4 || &data[0..4] != M8_MAGIC {
            return Err(anyhow::anyhow!("Invalid M8 container format"));
        }
        
        let mut cursor = 4;
        
        // Read header
        let header_len = u32::from_le_bytes([
            data[cursor], data[cursor+1], data[cursor+2], data[cursor+3]
        ]) as usize;
        cursor += 4;
        let header: M8Header = bincode::deserialize(&data[cursor..cursor+header_len])?;
        cursor += header_len;
        
        // Read wave signature
        let mut wave_signature = [0u8; 32];
        wave_signature.copy_from_slice(&data[cursor..cursor+32]);
        cursor += 32;
        
        // Read data length and data
        let data_len = u64::from_le_bytes([
            data[cursor], data[cursor+1], data[cursor+2], data[cursor+3],
            data[cursor+4], data[cursor+5], data[cursor+6], data[cursor+7]
        ]) as usize;
        cursor += 8;
        
        let container_data = data[cursor..cursor+data_len].to_vec();
        
        Ok(M8Container {
            header,
            data: container_data,
            wave_signature,
        })
    }
    
    /// Calculate SHA256 signature
    fn calculate_signature(data: &[u8]) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
    
    /// Extract content based on type
    pub fn extract_content(&self) -> Result<String> {
        match self.header.content_type {
            M8ContentType::Marqant => {
                let marqant = Marqant::from_bytes(&self.data)?;
                marqant.to_markdown()
            }
            M8ContentType::Language => {
                Ok(String::from_utf8_lossy(&self.data).to_string())
            }
            _ => {
                Ok(format!("M8 Container: {:?} with {} bytes of data", 
                    self.header.content_type, self.data.len()))
            }
        }
    }
}

/// M8 nexus - manages the quantum context storage
pub struct M8Nexus {
    mem8: Arc<Mutex<Mem8>>,
    containers: HashMap<[u8; 32], M8Container>, // Wave signature -> container
}

impl M8Nexus {
    pub fn new(mem8: Arc<Mutex<Mem8>>) -> Self {
        Self {
            mem8,
            containers: HashMap::new(),
        }
    }
    
    /// Store a container and return its wave signature
    pub fn store(&mut self, container: M8Container) -> [u8; 32] {
        let signature = container.wave_signature;
        self.containers.insert(signature, container);
        signature
    }
    
    /// Retrieve a container by wave signature
    pub fn retrieve(&self, signature: &[u8; 32]) -> Option<&M8Container> {
        self.containers.get(signature)
    }
    
    /// List all containers with metadata
    pub fn list(&self) -> Vec<([u8; 32], M8ContentType, std::time::SystemTime)> {
        self.containers.iter()
            .map(|(sig, container)| (*sig, container.header.content_type.clone(), container.header.timestamp))
            .collect()
    }
    
    /// Get nexus statistics
    pub fn stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("total_containers".to_string(), self.containers.len());
        
        let mut type_counts = HashMap::new();
        for container in self.containers.values() {
            let type_name = format!("{:?}", container.header.content_type);
            *type_counts.entry(type_name).or_insert(0) += 1;
        }
        
        for (type_name, count) in type_counts {
            stats.insert(format!("type_{}", type_name.to_lowercase()), count);
        }
        
        stats
    }
}

// Extension trait for EmotionalContext
impl EmotionalContext {
    fn to_bytes(&self) -> [u8; 3] {
        // Simple encoding: valence, arousal, dominance as 0-255
        [
            ((self.valence + 1.0) * 127.5) as u8,
            ((self.arousal + 1.0) * 127.5) as u8,
            ((self.dominance + 1.0) * 127.5) as u8,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_m8_text_container() {
        let mem8 = Arc::new(Mutex::new(Mem8::new_default().unwrap()));
        let container = M8Container::from_text("Hello, quantum world!", 5, mem8).unwrap();
        
        let serialized = container.to_bytes().unwrap();
        let deserialized = M8Container::from_bytes(&serialized).unwrap();
        
        assert_eq!(container.wave_signature, deserialized.wave_signature);
        assert_eq!(container.extract_content().unwrap(), "Hello, quantum world!");
    }
}