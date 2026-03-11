//! List command for the Zenobuf CLI

use clap::Subcommand;
use console::style;
use std::collections::BTreeSet;
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
        ListCommands::Nodes => list_by_prefix("Nodes", "zenobuf/node/").await,
        ListCommands::Topics => list_by_prefix("Topics", "zenobuf/topic/").await,
        ListCommands::Services => list_by_prefix("Services", "zenobuf/service/").await,
    }
}

/// Queries Zenoh for all keys under the given prefix and prints the extracted names
async fn list_by_prefix(label: &str, prefix: &str) -> Result<()> {
    println!("{}", style(format!("{label}:")).bold());

    let session = zenoh::open(zenoh::config::Config::default()).await?;
    let selector = KeyExpr::try_from(format!("{prefix}**"))?;

    let mut names = BTreeSet::new();
    let replies = session.get(selector).await?;

    while let Ok(reply) = replies.recv_async().await {
        if let Ok(sample) = reply.result() {
            if let Some(name) = sample.key_expr().as_str().strip_prefix(prefix) {
                names.insert(name.to_string());
            }
        }
    }

    if names.is_empty() {
        println!("  No {label} found", label = label.to_lowercase());
    } else {
        for name in names {
            println!("  {name}");
        }
    }

    Ok(())
}
