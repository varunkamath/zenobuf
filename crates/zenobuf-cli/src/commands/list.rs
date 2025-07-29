//! List command for the Zenobuf CLI

use clap::Subcommand;
use console::style;
use std::collections::HashSet;
use zenoh::{self, key_expr::KeyExpr};

use crate::error::Result;

/// Subcommands for the list command
#[derive(Subcommand)]
pub enum ListCommands {
    /// List nodes
    Nodes,
    /// List topics
    Topics,
    /// List services
    Services,
}

/// Executes the list command
pub async fn execute(cmd: ListCommands) -> Result<()> {
    match cmd {
        ListCommands::Nodes => list_nodes().await,
        ListCommands::Topics => list_topics().await,
        ListCommands::Services => list_services().await,
    }
}

/// Lists all nodes
async fn list_nodes() -> Result<()> {
    println!("{}", style("Nodes:").bold());

    // Connect to Zenoh
    let session = zenoh::open(zenoh::config::Config::default()).await?;

    // Query for nodes using the node prefix
    let node_prefix = "zenobuf/node/**";
    let selector = KeyExpr::try_from(node_prefix)?;

    let mut nodes = HashSet::new();
    let replies = session.get(selector).await?;

    while let Ok(reply) = replies.recv_async().await {
        if let Ok(sample) = reply.result() {
            let key = sample.key_expr().as_str();
            // Extract node name from the key expression
            if let Some(node_name) = key.strip_prefix("zenobuf/node/") {
                nodes.insert(node_name.to_string());
            }
        }
    }

    if nodes.is_empty() {
        println!("  No nodes found");
    } else {
        for node in nodes {
            println!("  {node}");
        }
    }

    Ok(())
}

/// Lists all topics
async fn list_topics() -> Result<()> {
    println!("{}", style("Topics:").bold());

    // Connect to Zenoh
    let session = zenoh::open(zenoh::config::Config::default()).await?;

    // Query for topics using the topic prefix
    let topic_prefix = "zenobuf/topic/**";
    let selector = KeyExpr::try_from(topic_prefix)?;

    let mut topics = HashSet::new();
    let replies = session.get(selector).await?;

    while let Ok(reply) = replies.recv_async().await {
        if let Ok(sample) = reply.result() {
            let key = sample.key_expr().as_str();
            // Extract topic name from the key expression
            if let Some(topic_name) = key.strip_prefix("zenobuf/topic/") {
                topics.insert(topic_name.to_string());
            }
        }
    }

    if topics.is_empty() {
        println!("  No topics found");
    } else {
        for topic in topics {
            println!("  {topic}");
        }
    }

    Ok(())
}

/// Lists all services
async fn list_services() -> Result<()> {
    println!("{}", style("Services:").bold());

    // Connect to Zenoh
    let session = zenoh::open(zenoh::config::Config::default()).await?;

    // Query for services using the service prefix
    let service_prefix = "zenobuf/service/**";
    let selector = KeyExpr::try_from(service_prefix)?;

    let mut services = HashSet::new();
    let replies = session.get(selector).await?;

    while let Ok(reply) = replies.recv_async().await {
        if let Ok(sample) = reply.result() {
            let key = sample.key_expr().as_str();
            // Extract service name from the key expression
            if let Some(service_name) = key.strip_prefix("zenobuf/service/") {
                services.insert(service_name.to_string());
            }
        }
    }

    if services.is_empty() {
        println!("  No services found");
    } else {
        for service in services {
            println!("  {service}");
        }
    }

    Ok(())
}
