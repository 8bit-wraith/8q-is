//! Live Feed Context Auctioneer
//!
//! "Going once, going twice, SOLD to the quantum field!" - The Auctioneer
//!
//! Real-time commentary on memory contributions and context changes

use std::sync::{Arc, Mutex};
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, Duration};
use mem8::{Mem8, EmotionalContext};
use crate::m8::{M8Container, M8ContentType};
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};

/// Contributor stats for the leaderboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributorStats {
    pub id: String,
    pub name: String,
    pub total_contributions: u64,
    pub wave_strength: f32,
    pub emotional_impact: f32,
    pub last_seen: SystemTime,
    pub specialty: ContributorType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributorType {
    TextMaster,      // Contributes lots of text/language
    WaveWhisperer,   // Strong wave patterns
    EmotionEngine,   // High emotional context
    QuantumQueen,    // Marqant compression expert
    MemoryMogul,     // Stores many memories
}

/// Live event types for the feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuctionEvent {
    NewContribution {
        contributor: String,
        content_type: M8ContentType,
        wave_strength: f32,
        timestamp: SystemTime,
    },
    EmotionalSpike {
        contributor: String,
        emotion: EmotionalContext,
        intensity: f32,
    },
    QuantumEntanglement {
        contributors: Vec<String>,
        correlation: f32,
    },
    LeaderboardUpdate {
        top_contributors: Vec<ContributorStats>,
    },
    AuctioneerComment {
        message: String,
        excitement_level: u8, // 1-10
    },
    AIBattle {
        event: crate::auctioneer_battles::CodingBattleEvent,
    },
}

/// The Auctioneer - commentates on the quantum memory auction
pub struct Auctioneer {
    contributors: Arc<Mutex<HashMap<String, ContributorStats>>>,
    event_history: Arc<Mutex<VecDeque<AuctionEvent>>>,
    event_tx: mpsc::UnboundedSender<AuctionEvent>,
    excitement_threshold: f32,
    commentary_style: CommentaryStyle,
}

#[derive(Debug, Clone)]
pub enum CommentaryStyle {
    FastTalking,     // Classic auctioneer rapid-fire
    Dramatic,        // Theater-style dramatic pauses
    Technical,       // Explains the quantum mechanics
    Comedic,         // Lots of quantum puns
    Philosophical,   // Deep thoughts about consciousness
}

impl Auctioneer {
    pub fn new(event_tx: mpsc::UnboundedSender<AuctionEvent>) -> Self {
        Self {
            contributors: Arc::new(Mutex::new(HashMap::new())),
            event_history: Arc::new(Mutex::new(VecDeque::with_capacity(1000))),
            event_tx,
            excitement_threshold: 0.7,
            commentary_style: CommentaryStyle::FastTalking,
        }
    }

    /// Process a new M8 container and generate commentary
    pub async fn process_contribution(
        &self, 
        contributor_id: &str,
        container: &M8Container,
        _mem8: Arc<Mutex<Mem8>>
    ) {
        // Update contributor stats
        let wave_strength = self.calculate_wave_strength(&container.wave_signature);
        let emotional_impact = self.calculate_emotional_impact(&container.header.emotional_context);
        
        {
            let mut contributors = self.contributors.lock().unwrap();
            let contributor = contributors.entry(contributor_id.to_string())
                .or_insert_with(|| ContributorStats {
                    id: contributor_id.to_string(),
                    name: self.generate_quantum_name(contributor_id),
                    total_contributions: 0,
                    wave_strength: 0.0,
                    emotional_impact: 0.0,
                    last_seen: SystemTime::now(),
                    specialty: ContributorType::MemoryMogul,
                });
            
            contributor.total_contributions += 1;
            contributor.wave_strength = (contributor.wave_strength + wave_strength) / 2.0;
            contributor.emotional_impact = (contributor.emotional_impact + emotional_impact) / 2.0;
            contributor.last_seen = SystemTime::now();
            contributor.specialty = self.determine_specialty(contributor);
        }
        
        // Generate event
        let event = AuctionEvent::NewContribution {
            contributor: contributor_id.to_string(),
            content_type: container.header.content_type.clone(),
            wave_strength,
            timestamp: SystemTime::now(),
        };
        
        self.add_event(event.clone());
        let _ = self.event_tx.send(event);
        
        // Generate commentary
        self.generate_commentary(contributor_id, wave_strength, &container.header.content_type).await;
        
        // Check for special events
        self.check_for_quantum_events().await;
    }

    /// Generate auctioneer commentary based on the contribution
    async fn generate_commentary(&self, contributor_id: &str, wave_strength: f32, content_type: &M8ContentType) {
        let excitement = (wave_strength * 10.0).min(10.0) as u8;
        let contributor_name = self.contributors.lock().unwrap()
            .get(contributor_id)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| contributor_id.to_string());
        
        let message = match self.commentary_style {
            CommentaryStyle::FastTalking => {
                match (excitement, content_type) {
                    (8..=10, M8ContentType::Marqant) => 
                        format!("HOLYMEMORY! {contributor_name} just dropped a QUANTUM COMPRESSED MARQANT! \
                                Wave strength {wave_strength:.2} and climbing! Do I hear {:.2}? {:.2}? \
                                Going to the quantum field in 3... 2...", wave_strength + 0.1, wave_strength + 0.2),
                    (8..=10, _) => 
                        format!("WE GOT A HOT ONE! {contributor_name} coming in STRONG at {wave_strength:.2}! \
                                Can anyone match this intensity? Going once at {wave_strength:.2}!"),
                    (5..=7, _) => 
                        format!("Nice contribution from {contributor_name}! Solid {wave_strength:.2} on the wave meter! \
                                Who's gonna bump it up? I see potential for entanglement here!"),
                    _ => 
                        format!("{contributor_name} adds to the quantum pool at {wave_strength:.2}. \
                                Every wave counts in the memory nexus!")
                }
            },
            CommentaryStyle::Dramatic => {
                format!("*dramatic pause* ... {contributor_name} ... has entered the quantum realm ... \
                        *whispers* wave strength {wave_strength:.2} ... *SHOUTS* MAGNIFICENT!")
            },
            CommentaryStyle::Technical => {
                format!("Analyzing quantum signature from {contributor_name}: Wave amplitude {wave_strength:.2}, \
                        Type: {:?}, Estimated coherence time: {:.2}ms. Fascinating quantum superposition detected!",
                        content_type, wave_strength * 1000.0)
            },
            CommentaryStyle::Comedic => {
                format!("Well well well, if it isn't {contributor_name} with a {wave_strength:.2}! \
                        That's more entangled than my headphones after a day in my pocket! \
                        Schrödinger's cat is definitely alive after seeing this one!")
            },
            CommentaryStyle::Philosophical => {
                format!("In the grand tapestry of consciousness, {contributor_name} weaves a thread of {wave_strength:.2}. \
                        What is memory but waves upon the ocean of mind? This {:?} speaks to the eternal now...",
                        content_type)
            }
        };
        
        let comment_event = AuctionEvent::AuctioneerComment { message, excitement_level: excitement };
        let _ = self.event_tx.send(comment_event);
    }

    /// Check for quantum entanglement between contributors
    async fn check_for_quantum_events(&self) {
        let contributors = self.contributors.lock().unwrap();
        
        // Find recent active contributors
        let now = SystemTime::now();
        let recent: Vec<_> = contributors.values()
            .filter(|c| {
                c.last_seen.duration_since(SystemTime::UNIX_EPOCH).unwrap()
                    .saturating_sub(now.duration_since(SystemTime::UNIX_EPOCH).unwrap())
                    < Duration::from_secs(30)
            })
            .collect();
        
        // Check for quantum entanglement (similar wave strengths)
        for i in 0..recent.len() {
            for j in i+1..recent.len() {
                let correlation = 1.0 - (recent[i].wave_strength - recent[j].wave_strength).abs();
                if correlation > 0.9 {
                    let event = AuctionEvent::QuantumEntanglement {
                        contributors: vec![recent[i].name.clone(), recent[j].name.clone()],
                        correlation,
                    };
                    let _ = self.event_tx.send(event);
                    
                    // Special commentary for entanglement
                    let message = format!("🌀 QUANTUM ENTANGLEMENT DETECTED! {} and {} are vibrating at {:.1}% correlation! \
                                         This is RARE folks! Their waves are practically dancing together!",
                                         recent[i].name, recent[j].name, correlation * 100.0);
                    let _ = self.event_tx.send(AuctionEvent::AuctioneerComment {
                        message,
                        excitement_level: 10,
                    });
                }
            }
        }
        
        // Update leaderboard
        self.update_leaderboard().await;
    }

    /// Update and broadcast the leaderboard
    async fn update_leaderboard(&self) {
        let mut contributors: Vec<_> = self.contributors.lock().unwrap()
            .values()
            .cloned()
            .collect();
        
        // Sort by combined score
        contributors.sort_by(|a, b| {
            let score_a = a.wave_strength * 0.4 + a.emotional_impact * 0.3 + (a.total_contributions as f32 * 0.01);
            let score_b = b.wave_strength * 0.4 + b.emotional_impact * 0.3 + (b.total_contributions as f32 * 0.01);
            score_b.partial_cmp(&score_a).unwrap()
        });
        
        let top_contributors = contributors.into_iter().take(10).collect();
        let event = AuctionEvent::LeaderboardUpdate { top_contributors };
        let _ = self.event_tx.send(event);
    }

    /// Calculate wave strength from signature
    fn calculate_wave_strength(&self, signature: &[u8; 32]) -> f32 {
        // Sum bytes and normalize to 0-1 range
        let sum: u32 = signature.iter().map(|&b| b as u32).sum();
        (sum as f32) / (255.0 * 32.0)
    }

    /// Calculate emotional impact from context
    fn calculate_emotional_impact(&self, emotional_bytes: &[u8; 3]) -> f32 {
        // Convert back to emotional values and calculate magnitude
        let valence = (emotional_bytes[0] as f32 - 127.5) / 127.5;
        let arousal = (emotional_bytes[1] as f32 - 127.5) / 127.5;
        let dominance = (emotional_bytes[2] as f32 - 127.5) / 127.5;
        
        (valence.powi(2) + arousal.powi(2) + dominance.powi(2)).sqrt() / 1.732 // Normalize by sqrt(3)
    }

    /// Generate a fun quantum-themed name for contributors
    fn generate_quantum_name(&self, id: &str) -> String {
        let hash = id.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
        let adjectives = ["Quantum", "Entangled", "Superposed", "Coherent", "Spinning", "Probable", "Uncertain"];
        let nouns = ["Qubit", "Wave", "Photon", "Neutrino", "Boson", "Fermion", "Hadron"];
        
        let adj_idx = (hash % adjectives.len() as u32) as usize;
        let noun_idx = ((hash >> 8) % nouns.len() as u32) as usize;
        let number = hash % 1000;
        
        format!("{}{}{}", adjectives[adj_idx], nouns[noun_idx], number)
    }

    /// Determine contributor specialty based on their stats
    fn determine_specialty(&self, stats: &ContributorStats) -> ContributorType {
        if stats.wave_strength > 0.8 {
            ContributorType::WaveWhisperer
        } else if stats.emotional_impact > 0.7 {
            ContributorType::EmotionEngine
        } else if stats.total_contributions > 50 {
            ContributorType::MemoryMogul
        } else {
            ContributorType::TextMaster
        }
    }

    /// Add event to history
    fn add_event(&self, event: AuctionEvent) {
        let mut history = self.event_history.lock().unwrap();
        history.push_back(event);
        if history.len() > 1000 {
            history.pop_front();
        }
    }

    /// Get recent events
    pub fn get_recent_events(&self, count: usize) -> Vec<AuctionEvent> {
        self.event_history.lock().unwrap()
            .iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    /// Change commentary style
    pub fn set_style(&mut self, style: CommentaryStyle) {
        self.commentary_style = style;
    }
    
    /// Handle AI coding battle events
    pub async fn announce_battle_event(&self, battle_type: crate::auctioneer_battles::BattleEventType) {
        let (commentary, excitement) = crate::auctioneer_battles::generate_battle_commentary(&battle_type);
        let crowd_reaction = crate::auctioneer_battles::get_crowd_reaction(excitement);
        
        // Create the battle event
        let battle_event = crate::auctioneer_battles::CodingBattleEvent {
            timestamp: SystemTime::now(),
            event_type: battle_type,
            excitement_level: excitement,
            commentary: commentary.clone(),
        };
        
        // Send the battle event
        let _ = self.event_tx.send(AuctionEvent::AIBattle { event: battle_event });
        
        // Add crowd reaction as a follow-up comment
        let _ = self.event_tx.send(AuctionEvent::AuctioneerComment {
            message: format!("*The crowd reacts: {}*", crowd_reaction),
            excitement_level: excitement.saturating_sub(2),
        });
    }
}

/// WebSocket message for live updates
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveFeedMessage {
    pub event: AuctionEvent,
    pub timestamp: SystemTime,
    pub sequence: u64,
}