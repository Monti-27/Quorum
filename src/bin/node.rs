// node binary: grpc server that stores secret shares
// run multiple instances on different ports to form a custodian network

use std::env;
use std::sync::Arc;
use tonic::transport::Server;

use quorum::{ShareStore, CustodianService, CustodianServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parsing port from command line args
    // supports both: `node 50052` and `node --port 50052`
    let args: Vec<String> = env::args().collect();
    let port = parse_port(&args);

    fn parse_port(args: &[String]) -> u16 {
        for (i, arg) in args.iter().enumerate() {
            // check for --port flag
            if arg == "--port" || arg == "-p" {
                if let Some(port_str) = args.get(i + 1) {
                    return port_str.parse().unwrap_or(50051);
                }
            }
            // check for positional arg (first non-flag arg after binary name)
            if i == 1 && !arg.starts_with('-') {
                return arg.parse().unwrap_or(50051);
            }
        }
        50051 // default port
    }

    let addr = format!("0.0.0.0:{}", port).parse()?;
    let node_id = format!("node-{}", port);

    // creating the share store (thread-safe for concurrent requests)
    let store = Arc::new(ShareStore::new());

    // creating the custodian service
    let service = CustodianService::new(store, node_id.clone());

    println!("=== quorum custodian node ===");
    println!("[{}] listening on {}", node_id, addr);

    // starting the grpc server
    Server::builder()
        .add_service(CustodianServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
