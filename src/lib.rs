// quorum: threshold signature scheme engine
// implements shamir's secret sharing over secp256k1

pub mod core;
pub mod network;

// re-exporting the main types for convenient access
pub use core::scheme::{split_secret, recover_secret, Share, FragmentError};
pub use network::{ShareStore, CustodianService, CustodianServer, CustodianClient, ShareData, RetrieveRequest};

