// in-memory storage for secret shares
// uses thread-safe concurrent access for the grpc server

use std::collections::HashMap;
use std::sync::RwLock;
use k256::elliptic_curve::scalar::ScalarPrimitive;
use k256::{Scalar, Secp256k1};
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
    /// overwrites if the ceremony_id already exists
    pub fn store(&self, ceremony_id: String, share: Share) {
        let mut store = self.shares.write().unwrap();
        store.insert(ceremony_id, share);
    }

    /// retrieves a share for a given ceremony
    /// returns none if the ceremony_id doesn't exist
    pub fn retrieve(&self, ceremony_id: &str) -> Option<Share> {
        let store = self.shares.read().unwrap();
        store.get(ceremony_id).cloned()
    }

    /// checks if a share exists for a given ceremony
    pub fn exists(&self, ceremony_id: &str) -> bool {
        let store = self.shares.read().unwrap();
        store.contains_key(ceremony_id)
    }
}

impl Default for ShareStore {
    fn default() -> Self {
        Self::new()
    }
}

/// converts a k256 scalar to bytes for grpc transmission
pub fn scalar_to_bytes(scalar: &Scalar) -> Vec<u8> {
    scalar.to_bytes().to_vec()
}

/// converts bytes back to a k256 scalar
/// uses modular reduction to ensure the result is valid
pub fn bytes_to_scalar(bytes: &[u8]) -> Scalar {
    let arr: [u8; 32] = bytes.try_into().expect("invalid scalar bytes length");
    // using scalar primitive for safe conversion with modular reduction
    let primitive = ScalarPrimitive::<Secp256k1>::from_bytes((&arr).into()).unwrap();
    Scalar::from(primitive)
}

