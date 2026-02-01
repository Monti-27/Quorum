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
/// stores and retrieves secret shares for the client
pub struct CustodianService {
    store: Arc<ShareStore>,
    node_id: String,
}

impl CustodianService {
    /// creates a new custodian service with the given store
    pub fn new(store: Arc<ShareStore>, node_id: String) -> Self {
        Self { store, node_id }
    }
}

#[tonic::async_trait]
impl Custodian for CustodianService {
    /// handles a node joining a ceremony
    /// for now just acknowledges the join request
    async fn join_ceremony(
        &self,
        request: Request<JoinRequest>,
    ) -> Result<Response<JoinResponse>, Status> {
        let req = request.into_inner();
        println!("[{}] node {} joined ceremony", self.node_id, req.node_id);

        Ok(Response::new(JoinResponse {
            success: true,
            assigned_index: 0, // assigned by client in this simple implementation
            message: format!("welcome to the ceremony, {}", req.node_id),
        }))
    }

    /// stores a share sent by the client
    /// the share is kept in memory for later retrieval
    async fn store_share(
        &self,
        request: Request<ShareData>,
    ) -> Result<Response<StoreResponse>, Status> {
        let data = request.into_inner();
        
        // converting the bytes back to scalars
        let x = bytes_to_scalar(&data.x);
        let y = bytes_to_scalar(&data.y);
        let share = Share { x, y };

        // storing the share
        self.store.store(data.ceremony_id.clone(), share);
        
        println!(
            "[{}] stored share for ceremony '{}'",
            self.node_id, data.ceremony_id
        );

        Ok(Response::new(StoreResponse {
            success: true,
            message: "share stored successfully".to_string(),
        }))
    }

    /// retrieves a previously stored share
    /// called by the client during secret recovery
    async fn retrieve_share(
        &self,
        request: Request<RetrieveRequest>,
    ) -> Result<Response<ShareData>, Status> {
        let req = request.into_inner();
        
        // looking up the share
        match self.store.retrieve(&req.ceremony_id) {
            Some(share) => {
                println!(
                    "[{}] retrieved share for ceremony '{}'",
                    self.node_id, req.ceremony_id
                );

                Ok(Response::new(ShareData {
                    ceremony_id: req.ceremony_id,
                    x: scalar_to_bytes(&share.x),
                    y: scalar_to_bytes(&share.y),
                }))
            }
            None => {
                Err(Status::not_found(format!(
                    "no share found for ceremony '{}'",
                    req.ceremony_id
                )))
            }
        }
    }
}
