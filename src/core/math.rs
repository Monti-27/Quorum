// low-level math operations for shamir's secret sharing
// all computations stay within secp256k1's scalar field

use k256::Scalar;

/// evaluates a polynomial at point x using horner's method
/// coefficients are ordered from lowest to highest degree: [a0, a1, a2, ...]
/// returns: a0 + a1*x + a2*x^2 + ...
pub fn evaluate_polynomial(coefficients: &[Scalar], x: &Scalar) -> Scalar {
    // start from the highest degree coefficient and work backwards
    // this is horner's method: ((a_n * x + a_{n-1}) * x + ...) * x + a_0
    coefficients
        .iter()
        .rev()
        .fold(Scalar::ZERO, |acc, coeff| acc * x + coeff)
}
