// grpc service implementation for the custodian node
// handles incoming requests from the client coordinator

use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::core::scheme::Share;
use crate::network::storage::{ShareStore, scalar_to_bytes, bytes_to_scalar};

// import the generated protobuf types
pub mod proto {
    tonic::include_proto!("custodian");
}

use proto::custodian_server::Custodian;
use proto::{JoinRequest, JoinResponse, ShareData, StoreResponse, RetrieveRequest};

/// the custodian service that runs on each node
pub struct CustodianService {
    store: Arc<ShareStore>,
    node_id: String,
}

impl CustodianService {
    pub fn new(store: Arc<ShareStore>, node_id: String) -> Self {
        Self { store, node_id }
    }
}
