// in-memory storage for secret shares
// uses thread-safe concurrent access for the grpc server

use std::collections::HashMap;
use std::sync::RwLock;
use crate::core::scheme::Share;

/// stores shares keyed by ceremony id
/// thread-safe for concurrent grpc requests
pub struct ShareStore {
    // maps ceremony_id -> Share
    shares: RwLock<HashMap<String, Share>>,
}

impl ShareStore {
    /// creates a new empty share store
    pub fn new() -> Self {
        Self {
            shares: RwLock::new(HashMap::new()),
        }
    }

    /// stores a share for a given ceremony
    pub fn store(&self, ceremony_id: String, share: Share) {
        let mut store = self.shares.write().unwrap();
        store.insert(ceremony_id, share);
    }

    /// retrieves a share for a given ceremony
    pub fn retrieve(&self, ceremony_id: &str) -> Option<Share> {
        let store = self.shares.read().unwrap();
        store.get(ceremony_id).cloned()
    }
}

impl Default for ShareStore {
    fn default() -> Self {
        Self::new()
    }
}
