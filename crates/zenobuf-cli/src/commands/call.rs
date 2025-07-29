//! Call command for the Zenobuf CLI

use clap::Args;
use console::style;
use serde_json::{json, Value};
use zenoh::{self, key_expr::KeyExpr};

use crate::error::Result;

/// Arguments for the call command
#[derive(Args)]
pub struct CallArgs {
    /// Service to call
    service: String,

    /// Request data (JSON)
    #[clap(short, long)]
    data: Option<String>,

    /// Timeout in seconds
    #[clap(short, long, default_value = "5")]
    timeout: u64,
}

/// Executes the call command
pub async fn execute(args: CallArgs) -> Result<()> {
    println!("{} {}", style("Calling service:").bold(), args.service);

    // Parse the request data
    let request_data = match &args.data {
        Some(data) => {
            println!("  With data: {data}");
            serde_json::from_str::<Value>(data)?
        }
        None => {
            println!("  With empty data");
            json!({})
        }
    };

    // Connect to Zenoh
    let session = zenoh::open(zenoh::config::Config::default()).await?;

    // Create the full service path
    let service_path = format!("zenobuf/service/{}", args.service);
    let key_expr = KeyExpr::try_from(service_path)?;

    // Serialize the request data
    let request_bytes = serde_json::to_vec(&request_data)?;

    // Call the service
    println!("  Waiting for response...");
    let timeout = std::time::Duration::from_secs(args.timeout);
    let replies = session
        .get(key_expr)
        .payload(request_bytes)
        .timeout(timeout)
        .await?;

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
                            println!("\n{}", style("Response:").bold());
                            println!("{}", serde_json::to_string_pretty(&json)?);
                        }
                        Err(_) => {
                            // If not JSON, print as string
                            let payload_str = String::from_utf8_lossy(&payload);
                            println!("\n{}", style("Response:").bold());
                            println!("{payload_str}");
                        }
                    }
                }
                Err(e) => {
                    println!("\n{}", style("Error:").bold().red());
                    println!("  {e}");
                }
            }
        }
        Err(e) => {
            println!("\n{}", style("Error:").bold().red());
            println!("  {e}");
        }
    }

    Ok(())
}
