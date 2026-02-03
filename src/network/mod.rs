// network module exports
// provides grpc service and storage components

pub mod storage;
pub mod service;

// re-export key types for convenience
pub use storage::ShareStore;
pub use service::CustodianService;
pub use service::proto::custodian_server::CustodianServer;
pub use service::proto::custodian_client::CustodianClient;
pub use service::proto::{ShareData, RetrieveRequest, JoinRequest};
