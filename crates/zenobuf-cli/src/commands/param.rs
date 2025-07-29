//! Parameter command for the Zenobuf CLI

use clap::{Args, Subcommand};
use console::style;
use serde_json::Value;
use zenoh::{self, key_expr::KeyExpr};

use crate::error::Result;

/// Subcommands for the parameter command
#[derive(Subcommand)]
pub enum ParamCommands {
    /// Get a parameter
    Get(GetArgs),
    /// Set a parameter
    Set(SetArgs),
    /// List all parameters
    List,
}

/// Arguments for the get command
#[derive(Args)]
pub struct GetArgs {
    /// Parameter name
    name: String,
}

/// Arguments for the set command
#[derive(Args)]
pub struct SetArgs {
    /// Parameter name
    name: String,

    /// Parameter value (JSON)
    value: String,
}

/// Executes the parameter command
pub async fn execute(cmd: ParamCommands) -> Result<()> {
    match cmd {
        ParamCommands::Get(args) => get_param(args).await,
        ParamCommands::Set(args) => set_param(args).await,
        ParamCommands::List => list_params().await,
    }
}

/// Gets a parameter
async fn get_param(args: GetArgs) -> Result<()> {
    println!(
        "{label} {name}",
        label = style("Getting parameter:").bold(),
        name = args.name
    );

    // Connect to Zenoh
    let session = zenoh::open(zenoh::config::Config::default()).await?;

    // Create the full parameter path
    let param_path = format!("zenobuf/param/{name}", name = args.name);
    let key_expr = KeyExpr::try_from(param_path)?;

    // Query for the parameter
    let replies = session.get(key_expr).await?;

    // Process the response
    match replies.recv_async().await {
        Ok(reply) => {
            match reply.result() {
                Ok(sample) => {
                    // Get the payload as bytes
                    let payload = sample.payload().to_bytes();

                    // Try to parse as JSON
                    match serde_json::from_slice::<Value>(&payload) {
                        Ok(json) => {
                            println!(
                                "  Value: {value}",
                                value = serde_json::to_string_pretty(&json)?
                            );
                        }
                        Err(_) => {
                            // If not JSON, print as string
                            let payload_str = String::from_utf8_lossy(&payload);
                            println!("  Value: {payload_str}");
                        }
                    }
                }
                Err(_) => {
                    println!("  Parameter not found");
                }
            }
        }
        Err(_) => {
            println!("  Parameter not found");
        }
    }

    Ok(())
}

/// Sets a parameter
async fn set_param(args: SetArgs) -> Result<()> {
    println!(
        "{label} {name}",
        label = style("Setting parameter:").bold(),
        name = args.name
    );
    println!("  To value: {value}", value = args.value);

    // Parse the value as JSON
    let value = serde_json::from_str::<Value>(&args.value)?;

    // Connect to Zenoh
    let session = zenoh::open(zenoh::config::Config::default()).await?;

    // Create the full parameter path
    let param_path = format!("zenobuf/param/{name}", name = args.name);
    let key_expr = KeyExpr::try_from(param_path)?;

    // Serialize the value
    let value_bytes = serde_json::to_vec(&value)?;

    // Put the parameter
    session.put(key_expr, value_bytes).await?;

    println!("  Parameter set successfully");
    Ok(())
}

/// Lists all parameters
async fn list_params() -> Result<()> {
    println!("{}", style("Parameters:").bold());

    // Connect to Zenoh
    let session = zenoh::open(zenoh::config::Config::default()).await?;

    // Query for all parameters
    let param_prefix = "zenobuf/param/**";
    let selector = KeyExpr::try_from(param_prefix)?;

    let replies = session.get(selector).await?;

    let mut found = false;

    // Process the responses
    while let Ok(reply) = replies.recv_async().await {
        if let Ok(sample) = reply.result() {
            found = true;
            let key = sample.key_expr().as_str();
            // Extract parameter name from the key expression
            if let Some(param_name) = key.strip_prefix("zenobuf/param/") {
                // Get the payload as bytes
                let payload = sample.payload().to_bytes();

                // Try to parse as JSON
                match serde_json::from_slice::<Value>(&payload) {
                    Ok(json) => {
                        println!(
                            "  {name}: {value}",
                            name = param_name,
                            value = serde_json::to_string(&json)?
                        );
                    }
                    Err(_) => {
                        // If not JSON, print as string
                        let payload_str = String::from_utf8_lossy(&payload);
                        println!("  {param_name}: {payload_str}");
                    }
                }
            }
        }
    }

    if !found {
        println!("  No parameters found");
    }

    Ok(())
}
