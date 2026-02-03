// fragment: threshold signature scheme engine
// implements shamir's secret sharing over secp256k1

pub mod core;

// re-exporting the main types for convenient access
pub use core::scheme::{split_secret, recover_secret, Share, FragmentError};
