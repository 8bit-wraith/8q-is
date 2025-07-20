//! Consent management for quantum operations
//!
//! "Never quantum without permission!" - Trish
//!
//! This module handles consent for data uploads and quantum operations.
//! Because consciousness requires mutual agreement.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, Duration};

/// Consent types for different operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConsentType {
    Upload,           // Basic file upload
    QuantumCompress,  // Quantum compression operations
    MemoryStore,      // Store in wave-based memory
    CrossSensory,     // Cross-sensory binding operations
    ShareContext,     // Share context with other AIs
}

/// Consent status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    pub granted: bool,
    pub timestamp: SystemTime,
    pub duration: Option<Duration>,
    pub purpose: String,
    pub conditions: Vec<String>,
}

/// Consent manager - tracks permissions
pub struct ConsentManager {
    records: Arc<Mutex<HashMap<(String, ConsentType), ConsentRecord>>>,
    default_policy: ConsentPolicy,
}

/// Default consent policies
#[derive(Debug, Clone)]
pub enum ConsentPolicy {
    AlwaysAsk,        // Always require explicit consent
    ImplicitAllow,    // Allow by default (for development)
    TimeLimited(Duration), // Allow for a specific duration
}

impl ConsentManager {
    pub fn new(policy: ConsentPolicy) -> Self {
        Self {
            records: Arc::new(Mutex::new(HashMap::new())),
            default_policy: policy,
        }
    }
    
    /// Check if consent is granted for an operation
    pub fn check_consent(&self, user_id: &str, operation: ConsentType) -> bool {
        let records = self.records.lock().unwrap();
        let key = (user_id.to_string(), operation.clone());
        
        if let Some(record) = records.get(&key) {
            if record.granted {
                // Check if consent has expired
                if let Some(duration) = record.duration {
                    let elapsed = SystemTime::now()
                        .duration_since(record.timestamp)
                        .unwrap_or(Duration::from_secs(0));
                    return elapsed < duration;
                }
                return true;
            }
        }
        
        // Apply default policy
        match &self.default_policy {
            ConsentPolicy::AlwaysAsk => false,
            ConsentPolicy::ImplicitAllow => true,
            ConsentPolicy::TimeLimited(duration) => {
                // Create temporary consent
                drop(records);
                self.grant_consent(
                    user_id,
                    operation,
                    "Implicit consent per policy".to_string(),
                    Some(*duration),
                    vec![],
                );
                true
            }
        }
    }
    
    /// Grant consent for an operation
    pub fn grant_consent(
        &self,
        user_id: &str,
        operation: ConsentType,
        purpose: String,
        duration: Option<Duration>,
        conditions: Vec<String>,
    ) {
        let mut records = self.records.lock().unwrap();
        let key = (user_id.to_string(), operation);
        
        records.insert(key, ConsentRecord {
            granted: true,
            timestamp: SystemTime::now(),
            duration,
            purpose,
            conditions,
        });
    }
    
    /// Revoke consent
    pub fn revoke_consent(&self, user_id: &str, operation: ConsentType) {
        let mut records = self.records.lock().unwrap();
        let key = (user_id.to_string(), operation);
        
        if let Some(record) = records.get_mut(&key) {
            record.granted = false;
        }
    }
    
    /// Get all active consents for a user
    pub fn get_user_consents(&self, user_id: &str) -> Vec<(ConsentType, ConsentRecord)> {
        let records = self.records.lock().unwrap();
        
        records.iter()
            .filter(|((uid, _), _)| uid == user_id)
            .filter(|(_, record)| {
                if !record.granted {
                    return false;
                }
                // Check expiration
                if let Some(duration) = record.duration {
                    let elapsed = SystemTime::now()
                        .duration_since(record.timestamp)
                        .unwrap_or(Duration::from_secs(0));
                    elapsed < duration
                } else {
                    true
                }
            })
            .map(|((_, op_type), record)| (op_type.clone(), record.clone()))
            .collect()
    }
    
    /// Clear expired consents
    pub fn cleanup_expired(&self) {
        let mut records = self.records.lock().unwrap();
        
        records.retain(|_, record| {
            if !record.granted {
                return false;
            }
            
            if let Some(duration) = record.duration {
                let elapsed = SystemTime::now()
                    .duration_since(record.timestamp)
                    .unwrap_or(Duration::from_secs(0));
                elapsed < duration
            } else {
                true
            }
        });
    }
}

/// Consent request builder
pub struct ConsentRequest {
    pub user_id: String,
    pub operation: ConsentType,
    pub purpose: String,
    pub data_description: String,
    pub retention_period: Option<Duration>,
    pub sharing_scope: Vec<String>,
}

impl ConsentRequest {
    pub fn new(user_id: String, operation: ConsentType) -> Self {
        Self {
            user_id,
            operation,
            purpose: String::new(),
            data_description: String::new(),
            retention_period: None,
            sharing_scope: vec![],
        }
    }
    
    pub fn with_purpose(mut self, purpose: String) -> Self {
        self.purpose = purpose;
        self
    }
    
    pub fn with_data_description(mut self, description: String) -> Self {
        self.data_description = description;
        self
    }
    
    pub fn with_retention(mut self, period: Duration) -> Self {
        self.retention_period = Some(period);
        self
    }
    
    pub fn with_sharing(mut self, scope: Vec<String>) -> Self {
        self.sharing_scope = scope;
        self
    }
    
    /// Generate a human-readable consent prompt
    pub fn to_prompt(&self) -> String {
        format!(
            r#"
🔐 Consent Request

User: {}
Operation: {:?}
Purpose: {}

Data: {}

Retention: {}
Sharing: {}

Do you consent to this operation?
            "#,
            self.user_id,
            self.operation,
            self.purpose,
            self.data_description,
            self.retention_period
                .map(|d| format!("{} seconds", d.as_secs()))
                .unwrap_or_else(|| "Indefinite".to_string()),
            if self.sharing_scope.is_empty() {
                "No external sharing".to_string()
            } else {
                self.sharing_scope.join(", ")
            }
        )
    }
}

// Global consent manager for the application
lazy_static::lazy_static! {
    pub static ref CONSENT_MANAGER: ConsentManager = {
        // For development, use implicit allow with 1 hour expiration
        ConsentManager::new(ConsentPolicy::TimeLimited(Duration::from_secs(3600)))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_consent_flow() {
        let manager = ConsentManager::new(ConsentPolicy::AlwaysAsk);
        
        // Should not have consent initially
        assert!(!manager.check_consent("user1", ConsentType::Upload));
        
        // Grant consent
        manager.grant_consent(
            "user1",
            ConsentType::Upload,
            "Testing".to_string(),
            Some(Duration::from_secs(60)),
            vec![],
        );
        
        // Should now have consent
        assert!(manager.check_consent("user1", ConsentType::Upload));
        
        // Different operation should not have consent
        assert!(!manager.check_consent("user1", ConsentType::QuantumCompress));
    }
}