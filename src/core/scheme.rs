// high-level secret sharing scheme
// implements split and recover operations for shamir's secret sharing

use k256::Scalar;
use rand::rngs::OsRng;
use thiserror::Error;

use crate::core::math::{evaluate_polynomial, lagrange_interpolate};

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
/// 
/// - secret: the value to split (typically a private key as a scalar)
/// - threshold: minimum number of shares needed to reconstruct
/// - total_shares: total number of shares to generate
/// 
/// returns a vector of shares, any `threshold` of which can reconstruct the secret
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
    // we need (threshold - 1) random coefficients for degrees 1 to (threshold - 1)
    let mut coefficients = vec![*secret];
    let mut rng = OsRng;
    
    for _ in 1..threshold {
        // generating a random scalar using k256's built-in method
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

/// reconstructs the secret from a set of shares using lagrange interpolation
/// 
/// - shares: the shares to use for reconstruction (must be at least threshold shares)
/// 
/// returns the original secret if enough valid shares are provided
pub fn recover_secret(shares: &[Share]) -> Result<Scalar, FragmentError> {
    if shares.len() < 2 {
        return Err(FragmentError::InsufficientShares);
    }

    // converting shares to the format expected by lagrange interpolation
    let points: Vec<(Scalar, Scalar)> = shares.iter().map(|s| (s.x, s.y)).collect();

    // reconstructing the secret (y-intercept at x=0)
    let secret = lagrange_interpolate(&points);

    Ok(secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_and_recover() {
        // creating a known secret
        let secret = Scalar::from(123456789u64);

        // splitting into 5 shares with threshold of 3
        let shares = split_secret(&secret, 3, 5).unwrap();
        assert_eq!(shares.len(), 5);

        // recovering with exactly 3 shares
        let recovered = recover_secret(&shares[0..3]).unwrap();
        assert_eq!(recovered, secret);

        // recovering with different 3 shares should also work
        let recovered2 = recover_secret(&shares[2..5]).unwrap();
        assert_eq!(recovered2, secret);
    }

    #[test]
    fn test_invalid_threshold() {
        let secret = Scalar::from(42u64);

        // threshold less than 2
        assert!(split_secret(&secret, 1, 5).is_err());

        // threshold greater than total
        assert!(split_secret(&secret, 6, 5).is_err());
    }
}
