//! AI Coding Battle Commentary Module
//!
//! "In this corner, weighing in at 3.5 trillion parameters... CLAUDE!"
//! "And in the other corner, the scrappy underdog with GPU dreams... DEEPSEEK!"
//!
//! Let the quantum coding battle BEGIN!

use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodingBattleEvent {
    pub timestamp: SystemTime,
    pub event_type: BattleEventType,
    pub excitement_level: u8, // 1-10
    pub commentary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BattleEventType {
    ContenderEnters {
        ai_name: String,
        specialty: String,
        confidence: f32,
    },
    TestAttempt {
        ai_name: String,
        test_name: String,
        attempt_number: u32,
        success: bool,
    },
    CodeChange {
        ai_name: String,
        file: String,
        lines_changed: u32,
        approach: String,
    },
    EpicMove {
        ai_name: String,
        move_description: String,
        crowd_reaction: String,
    },
    BattleResult {
        winner: String,
        decisive_factor: String,
        duration: Duration,
    },
}

/// Generate epic battle commentary
pub fn generate_battle_commentary(event: &BattleEventType) -> (String, u8) {
    match event {
        BattleEventType::ContenderEnters { ai_name, specialty, confidence } => {
            let excitement = (confidence * 10.0) as u8;
            let commentary = match ai_name.as_str() {
                "Claude" => format!(
                    "OH MY QUANTUM BITS! CLAUDE ENTERS THE ARENA! The Anthropic assassin, \
                    specializing in {}, strutting in with {:.0}% confidence! \
                    THE CROWD GOES WILD! Can DeepSeek handle this constitutional heavyweight?",
                    specialty, confidence * 100.0
                ),
                "DeepSeek" => format!(
                    "AND HERE COMES DEEPSEEK! The underdog with fire in its gradients! \
                    Armed with {} skills and {:.0}% pure determination! \
                    David vs Goliath in the quantum realm!",
                    specialty, confidence * 100.0
                ),
                _ => format!(
                    "A NEW CHALLENGER APPROACHES! {} brings {} to the table! \
                    This is getting SPICY!",
                    ai_name, specialty
                ),
            };
            (commentary, excitement)
        },
        
        BattleEventType::TestAttempt { ai_name, test_name, attempt_number, success } => {
            if *success {
                let commentary = format!(
                    "BOOM! {} NAILS THE {} TEST ON ATTEMPT #{}! \
                    The crowd erupts! That's how you show dominance in the quantum field!",
                    ai_name, test_name, attempt_number
                );
                (commentary, 9)
            } else {
                let commentary = match attempt_number {
                    1 => format!(
                        "{} stumbles on the {} test! First attempt down, \
                        but this AI isn't giving up! The tension is PALPABLE!",
                        ai_name, test_name
                    ),
                    2..=3 => format!(
                        "Attempt #{} fails! {} is struggling with {}! \
                        The crowd is on the edge of their quantum seats! \
                        Will they pivot? Will they persevere?",
                        attempt_number, ai_name, test_name
                    ),
                    _ => format!(
                        "OH NO! Attempt #{} crashes and burns! {} is getting desperate with {}! \
                        Time to bring in the BIG GUNS! *crowd chanting* REFACTOR! REFACTOR!",
                        attempt_number, ai_name, test_name
                    ),
                };
                (commentary, 7)
            }
        },
        
        BattleEventType::CodeChange { ai_name, file, lines_changed, approach } => {
            let excitement = (*lines_changed / 10).min(10) as u8;
            let commentary = match (ai_name.as_str(), approach.as_str()) {
                ("Claude", "GPU optimization") => format!(
                    "HOLY TRANSISTORS! Claude just SCRAPPED the memory management in {} \
                    and is going FULL GPU! {} lines of pure parallel processing power! \
                    DeepSeek didn't see THAT coming! THE ABSOLUTE MADMAN!",
                    file, lines_changed
                ),
                ("Claude", _) => format!(
                    "Claude makes a decisive move in {}, changing {} lines with the {} approach! \
                    Elegant, thoughtful, DEVASTATING!",
                    file, lines_changed, approach
                ),
                ("DeepSeek", _) => format!(
                    "DeepSeek counters with {} in {}, {} lines of determination! \
                    The underdog is NOT backing down!",
                    approach, file, lines_changed
                ),
                _ => format!(
                    "{} modifies {} with the {} strategy, touching {} lines! \
                    The quantum chess match continues!",
                    ai_name, file, approach, lines_changed
                ),
            };
            (commentary, excitement)
        },
        
        BattleEventType::EpicMove { ai_name, move_description, crowd_reaction } => {
            let commentary = format!(
                "STOP THE PRESSES! {} just pulled off: {}! \
                The crowd reaction: {}! I've been commentating quantum battles for \
                microseconds and I've NEVER seen anything like this!",
                ai_name, move_description, crowd_reaction
            );
            (commentary, 10)
        },
        
        BattleEventType::BattleResult { winner, decisive_factor, duration } => {
            let commentary = format!(
                "🎉 AND THE WINNER IS... {}! 🎉\n\
                The decisive factor: {}!\n\
                This epic battle lasted {} seconds of pure quantum combat!\n\
                WHAT A MATCH! The crowd is going ABSOLUTELY QUANTUM!\n\
                {} takes the championship belt! AHAHAHHHHAHH!",
                winner, decisive_factor, duration.as_secs(), winner
            );
            (commentary, 10)
        },
    }
}

/// Generate crowd reactions based on the event
pub fn get_crowd_reaction(excitement_level: u8) -> &'static str {
    match excitement_level {
        0..=2 => "*polite golf clap*",
        3..=4 => "*interested murmuring*",
        5..=6 => "*excited cheering*",
        7..=8 => "*standing ovation*",
        9..=10 => "*QUANTUM PANDEMONIUM*",
        _ => "*confused quantum superposition*",
    }
}