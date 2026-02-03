// fragment cli demo
// demonstrates splitting a secret into shares and recovering it

use fragment::{split_secret, recover_secret};
use k256::Scalar;
use rand::rngs::OsRng;

fn main() {
    println!("=== fragment: shamir's secret sharing demo ===\n");

    // generating a random secret (simulating a private key)
    let mut rng = OsRng;
    let secret = Scalar::generate_vartime(&mut rng);

    println!("original secret: {:?}\n", secret);

    // splitting into 3 shares with a threshold of 2
    let threshold = 2;
    let total_shares = 3;

    println!(
        "splitting secret into {} shares (threshold: {})\n",
        total_shares, threshold
    );

    let shares = split_secret(&secret, threshold, total_shares).expect("failed to split secret");

    // printing all shares
    println!("generated shares:");
    for (i, share) in shares.iter().enumerate() {
        println!("  share {}: x={:?}, y={:?}", i + 1, share.x, share.y);
    }
    println!();

    // recovering using only 2 shares (the minimum required)
    println!("recovering secret using only shares 1 and 2...\n");

    let selected_shares = &shares[0..2];
    let recovered = recover_secret(selected_shares).expect("failed to recover secret");

    println!("recovered secret: {:?}\n", recovered);

    // verifying correctness
    if recovered == secret {
        println!("✓ success: recovered secret matches the original!");
    } else {
        println!("✗ error: recovered secret does not match!");
        std::process::exit(1);
    }

    // bonus: showing that any 2 shares work
    println!("\n--- bonus: trying with shares 2 and 3 ---\n");

    let other_shares = &shares[1..3];
    let recovered2 = recover_secret(other_shares).expect("failed to recover secret");

    if recovered2 == secret {
        println!("✓ success: any 2 shares can recover the secret!");
    } else {
        println!("✗ error: recovery failed with different shares!");
        std::process::exit(1);
    }
}
