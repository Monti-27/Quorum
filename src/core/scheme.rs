// high-level secret sharing scheme
// implements split and recover operations for shamir's secret sharing

use k256::Scalar;
use rand::rngs::OsRng;
use thiserror::Error;

use crate::core::math::evaluate_polynomial;

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

/// splits a secret into multiple shares using shamir's secret sharing
pub fn split_secret(
    secret: &Scalar,
    threshold: usize,
    total_shares: usize,
) -> Result<Vec<Share>, FragmentError> {
    // sanity checks
    if threshold < 2 || threshold > total_shares {
        return Err(FragmentError::InvalidThreshold);
    }

    // generating random coefficients for the polynomial
    // the constant term (a0) is the secret itself
    let mut coefficients = vec![*secret];
    let mut rng = OsRng;
    
    for _ in 1..threshold {
        let random_coeff = Scalar::generate_vartime(&mut rng);
        coefficients.push(random_coeff);
    }

    // evaluating the polynomial at x = 1, 2, 3, ... to create shares
    let mut shares = Vec::with_capacity(total_shares);
    
    for i in 1..=total_shares {
        let x = Scalar::from(i as u64);
        let y = evaluate_polynomial(&coefficients, &x);
        shares.push(Share { x, y });
    }

    Ok(shares)
}
