// client binary: coordinator that distributes shares to custodian nodes
// connects to multiple nodes, splits a secret, distributes shares, then recovers

use k256::Scalar;
use rand::rngs::OsRng;
use tonic::transport::Channel;

use quorum::{split_secret, recover_secret, Share, CustodianClient, ShareData, RetrieveRequest};
use quorum::network::storage::{scalar_to_bytes, bytes_to_scalar};

/// connects to a custodian node at the given address
async fn connect_to_node(addr: &str) -> Result<CustodianClient<Channel>, Box<dyn std::error::Error>> {
    let client = CustodianClient::connect(addr.to_string()).await?;
    Ok(client)
}

/// stores a share on a remote custodian node
async fn store_share_on_node(
    client: &mut CustodianClient<Channel>,
    ceremony_id: &str,
    share: &Share,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = ShareData {
        ceremony_id: ceremony_id.to_string(),
        x: scalar_to_bytes(&share.x),
        y: scalar_to_bytes(&share.y),
    };

    let response = client.store_share(request).await?;
    println!("  -> {}", response.into_inner().message);
    Ok(())
}

/// retrieves a share from a remote custodian node
async fn retrieve_share_from_node(
    client: &mut CustodianClient<Channel>,
    ceremony_id: &str,
) -> Result<Share, Box<dyn std::error::Error>> {
    let request = RetrieveRequest {
        ceremony_id: ceremony_id.to_string(),
    };

    let response = client.retrieve_share(request).await?;
    let data = response.into_inner();

    let share = Share {
        x: bytes_to_scalar(&data.x),
        y: bytes_to_scalar(&data.y),
    };

    Ok(share)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== quorum client coordinator ===\n");

    // node addresses (run 3 nodes on these ports first)
    let node_addrs = vec![
        "http://127.0.0.1:50051",
        "http://127.0.0.1:50052",
        "http://127.0.0.1:50053",
    ];

    let threshold = 2;
    let total_shares = 3;
    let ceremony_id = "ceremony-001";

    // step 1: generate a random secret
    println!("step 1: generating random secret...");
    let mut rng = OsRng;
    let secret = Scalar::generate_vartime(&mut rng);
    println!("secret (hex): {}\n", hex::encode(secret.to_bytes()));

    // step 2: split the secret into shares
    println!("step 2: splitting secret into {} shares (threshold: {})...", total_shares, threshold);
    let shares = split_secret(&secret, threshold, total_shares)?;
    println!("generated {} shares\n", shares.len());

    // step 3: connect to nodes and distribute shares
    println!("step 3: distributing shares to custodian nodes...");
    let mut clients: Vec<CustodianClient<Channel>> = Vec::new();

    for (i, addr) in node_addrs.iter().enumerate() {
        print!("connecting to {}... ", addr);
        match connect_to_node(addr).await {
            Ok(mut client) => {
                println!("connected");
                // storing the share on this node
                store_share_on_node(&mut client, ceremony_id, &shares[i]).await?;
                clients.push(client);
            }
            Err(e) => {
                println!("failed: {}", e);
                return Err(e);
            }
        }
    }
    println!();

    // step 4: retrieve shares from threshold number of nodes
    println!("step 4: retrieving shares from {} nodes for recovery...", threshold);
    let mut recovered_shares: Vec<Share> = Vec::new();

    for (i, client) in clients.iter_mut().take(threshold).enumerate() {
        print!("retrieving from node {}... ", i + 1);
        let share = retrieve_share_from_node(client, ceremony_id).await?;
        println!("got share");
        recovered_shares.push(share);
    }
    println!();

    // step 5: recover the secret using lagrange interpolation
    println!("step 5: recovering secret using lagrange interpolation...");
    let recovered_secret = recover_secret(&recovered_shares)?;
    println!("recovered (hex): {}\n", hex::encode(recovered_secret.to_bytes()));

    // step 6: verify the recovery
    println!("step 6: verifying...");
    if recovered_secret == secret {
        println!("✓ success! recovered secret matches the original");
    } else {
        println!("✗ error! secrets do not match");
        std::process::exit(1);
    }

    Ok(())
}
