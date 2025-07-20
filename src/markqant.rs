//! Markqant (.mq) parsing and conversion logic
//!
//! "Quantum-compress your markdown, compress your worries!" - Hue
//!
//! Marqants are quantum-compressed markdown files that preserve meaning
//! while achieving massive compression through wave-based encoding.

// use mem8::{WavePattern};
use std::collections::HashMap;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use anyhow::Result;

/// Magic bytes for .mq files
pub const MQ_MAGIC: &[u8] = b"MQ03"; // Version 3 of Marqant format

/// Marqant header structure
#[derive(Debug, Serialize, Deserialize)]
pub struct MarqantHeader {
    pub version: u8,
    pub compression_level: u8,
    pub original_size: u64,
    pub compressed_size: u64,
    pub wave_signature: [u8; 32], // SHA256 of wave pattern
    pub metadata: HashMap<String, String>,
}

/// Marqant container - holds quantum-compressed markdown
#[derive(Debug)]
pub struct Marqant {
    pub header: MarqantHeader,
    pub wave_data: Vec<u8>,
    pub semantic_map: HashMap<String, Vec<u32>>, // Semantic tokens to wave indices
}

impl Marqant {
    /// Create a new Marqant from markdown text
    pub fn from_markdown(markdown: &str) -> Result<Self> {
        // Extract semantic tokens (headers, code blocks, links, etc.)
        let semantic_map = Self::extract_semantic_tokens(markdown);
        
        // Convert to wave pattern using quantum compression
        let wave_pattern = Self::markdown_to_wave(markdown, &semantic_map)?;
        
        // Compress the wave data
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&wave_pattern)?;
        let compressed_wave = encoder.finish()?;
        
        // Calculate wave signature
        let wave_signature = Self::calculate_wave_signature(&wave_pattern);
        
        let header = MarqantHeader {
            version: 3,
            compression_level: 9,
            original_size: markdown.len() as u64,
            compressed_size: compressed_wave.len() as u64,
            wave_signature,
            metadata: HashMap::from([
                ("format".to_string(), "marqant".to_string()),
                ("encoding".to_string(), "quantum-wave".to_string()),
            ]),
        };
        
        Ok(Marqant {
            header,
            wave_data: compressed_wave,
            semantic_map,
        })
    }
    
    /// Decompress and convert back to markdown
    pub fn to_markdown(&self) -> Result<String> {
        // Decompress wave data
        let mut decoder = ZlibDecoder::new(&self.wave_data[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        
        // Convert wave pattern back to markdown
        Self::wave_to_markdown(&decompressed, &self.semantic_map)
    }
    
    /// Extract semantic tokens from markdown
    fn extract_semantic_tokens(markdown: &str) -> HashMap<String, Vec<u32>> {
        let mut semantic_map = HashMap::new();
        let mut position = 0u32;
        
        // Extract headers
        let header_pattern = regex::Regex::new(r"^#{1,6}\s+(.+)$").unwrap();
        for line in markdown.lines() {
            if let Some(captures) = header_pattern.captures(line) {
                let _header = captures.get(1).unwrap().as_str();
                semantic_map.entry("headers".to_string())
                    .or_insert_with(Vec::new)
                    .push(position);
            }
            position += line.len() as u32 + 1; // +1 for newline
        }
        
        // Extract code blocks
        let code_blocks: Vec<_> = markdown.match_indices("```").collect();
        if code_blocks.len() >= 2 {
            semantic_map.insert("code_blocks".to_string(), 
                code_blocks.iter().map(|(pos, _)| *pos as u32).collect());
        }
        
        // Extract links
        let link_pattern = regex::Regex::new(r"\[([^\]]+)\]\(([^\)]+)\)").unwrap();
        let links: Vec<_> = link_pattern.find_iter(markdown)
            .map(|m| m.start() as u32)
            .collect();
        if !links.is_empty() {
            semantic_map.insert("links".to_string(), links);
        }
        
        semantic_map
    }
    
    /// Convert markdown to quantum wave pattern
    fn markdown_to_wave(markdown: &str, _semantic_map: &HashMap<String, Vec<u32>>) -> Result<Vec<u8>> {
        // For now, use a simple byte representation
        // In a full implementation, this would use quantum encoding
        // based on semantic understanding and wave interference patterns
        Ok(markdown.as_bytes().to_vec())
    }
    
    /// Convert wave pattern back to markdown
    fn wave_to_markdown(wave_data: &[u8], _semantic_map: &HashMap<String, Vec<u32>>) -> Result<String> {
        // For now, simple conversion
        // Full implementation would reconstruct using semantic map
        Ok(String::from_utf8_lossy(wave_data).to_string())
    }
    
    /// Calculate SHA256 signature of wave pattern
    fn calculate_wave_signature(wave_data: &[u8]) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(wave_data);
        hasher.finalize().into()
    }
    
    /// Serialize to .mq format
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        
        // Write magic bytes
        output.extend_from_slice(MQ_MAGIC);
        
        // Serialize header
        let header_bytes = bincode::serialize(&self.header)?;
        output.extend_from_slice(&(header_bytes.len() as u32).to_le_bytes());
        output.extend_from_slice(&header_bytes);
        
        // Serialize semantic map
        let semantic_bytes = bincode::serialize(&self.semantic_map)?;
        output.extend_from_slice(&(semantic_bytes.len() as u32).to_le_bytes());
        output.extend_from_slice(&semantic_bytes);
        
        // Add wave data
        output.extend_from_slice(&self.wave_data);
        
        Ok(output)
    }
    
    /// Deserialize from .mq format
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < 4 || &data[0..4] != MQ_MAGIC {
            return Err(anyhow::anyhow!("Invalid Marqant file format"));
        }
        
        let mut cursor = 4;
        
        // Read header
        let header_len = u32::from_le_bytes([
            data[cursor], data[cursor+1], data[cursor+2], data[cursor+3]
        ]) as usize;
        cursor += 4;
        let header: MarqantHeader = bincode::deserialize(&data[cursor..cursor+header_len])?;
        cursor += header_len;
        
        // Read semantic map
        let semantic_len = u32::from_le_bytes([
            data[cursor], data[cursor+1], data[cursor+2], data[cursor+3]
        ]) as usize;
        cursor += 4;
        let semantic_map: HashMap<String, Vec<u32>> = 
            bincode::deserialize(&data[cursor..cursor+semantic_len])?;
        cursor += semantic_len;
        
        // Remaining data is wave data
        let wave_data = data[cursor..].to_vec();
        
        Ok(Marqant {
            header,
            wave_data,
            semantic_map,
        })
    }
    
    /// Get compression ratio
    pub fn compression_ratio(&self) -> f64 {
        self.header.original_size as f64 / self.header.compressed_size as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_marqant_roundtrip() {
        let markdown = r#"# Test Document

This is a test document with **bold** and *italic* text.

## Code Example

```rust
fn main() {
    println!("Hello, quantum world!");
}
```

[Link to nowhere](https://example.com)
"#;
        
        let marqant = Marqant::from_markdown(markdown).unwrap();
        let recovered = marqant.to_markdown().unwrap();
        
        assert_eq!(markdown, recovered);
        println!("Compression ratio: {:.2}x", marqant.compression_ratio());
    }
}