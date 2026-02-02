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

/// reconstructs the secret (y-intercept at x=0) from a set of share points
/// uses lagrange interpolation to find the constant term of the polynomial
pub fn lagrange_interpolate(shares: &[(Scalar, Scalar)]) -> Scalar {
    let mut secret = Scalar::ZERO;

    for (i, (x_i, y_i)) in shares.iter().enumerate() {
        // computing the lagrange basis polynomial L_i(0)
        // L_i(0) = product of (-x_j / (x_i - x_j)) for all j != i
        let mut basis = Scalar::ONE;

        for (j, (x_j, _)) in shares.iter().enumerate() {
            if i != j {
                // numerator is -x_j (since we're evaluating at x=0)
                // denominator is (x_i - x_j)
                let numerator = Scalar::ZERO - x_j;
                let denominator = *x_i - x_j;

                // dividing in a finite field means multiplying by the inverse
                let denominator_inv = denominator.invert().unwrap();
                basis = basis * numerator * denominator_inv;
            }
        }

        // adding this share's contribution to the final result
        secret = secret + (*y_i * basis);
    }

    secret
}
