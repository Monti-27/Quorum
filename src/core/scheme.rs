// high-level secret sharing scheme
// implements split and recover operations for shamir's secret sharing

use k256::Scalar;
use thiserror::Error;

/// errors that can occur during secret sharing operations
#[derive(Error, Debug)]
pub enum FragmentError {
    #[error("threshold must be at least 2 and at most equal to total shares")]
    InvalidThreshold,

    #[error("not enough shares to recover the secret (need at least threshold shares)")]
    InsufficientShares,
}

/// represents a single share of the secret
/// x is the evaluation point, y is the polynomial value at that point
#[derive(Debug, Clone)]
pub struct Share {
    pub x: Scalar,
    pub y: Scalar,
}
