//! Monitor command for the Zenobuf CLI

use clap::Args;
use console::style;
use futures::StreamExt;
use serde_json::Value;
use tokio::pin;
use tokio::signal;
use zenoh::{self, key_expr::KeyExpr};

use crate::error::Result;

/// Arguments for the monitor command
#[derive(Args)]
pub struct MonitorArgs {
    /// Topic to monitor
    topic: String,

    /// Show timestamps
    #[clap(short, long)]
    timestamps: bool,

    /// Format output as JSON
    #[clap(short, long)]
    json: bool,
}

/// Executes the monitor command
pub async fn execute(args: MonitorArgs) -> Result<()> {
    println!("{} {}", style("Monitoring topic:").bold(), args.topic);
    println!("Press Ctrl+C to exit");

    // Connect to Zenoh
    let session = zenoh::open(zenoh::config::Config::default()).await?;

    // Create the full topic path
    let topic_path = format!("zenobuf/topic/{}", args.topic);
    let key_expr = KeyExpr::try_from(topic_path)?;

    // Subscribe to the topic
    let subscriber = session.declare_subscriber(key_expr).await?;

    // Create a stream from the subscriber
    let mut stream = subscriber.stream();

    // Create a signal handler for Ctrl+C
    let interrupt = signal::ctrl_c();
    pin!(interrupt);

    // Process messages until Ctrl+C is pressed
    loop {
        tokio::select! {
            _ = &mut interrupt => {
                println!("\nMonitoring stopped");
                break;
            }
            sample = stream.next() => {
                if let Some(sample) = sample {
                    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");

                    // Get the payload as bytes
                    let payload = sample.payload().to_bytes();

                    if args.json {
                        // Try to parse as JSON
                        if let Ok(json) = serde_json::from_slice::<Value>(&payload) {
                            if args.timestamps {
                                println!("{} {}", timestamp, serde_json::to_string_pretty(&json)?);
                            } else {
                                println!("{}", serde_json::to_string_pretty(&json)?);
                            }
                        } else {
                            // If not JSON, print as string
                            let payload_str = String::from_utf8_lossy(&payload);
                            if args.timestamps {
                                println!("{} {}", timestamp, payload_str);
                            } else {
                                println!("{}", payload_str);
                            }
                        }
                    } else {
                        // Print as string
                        let payload_str = String::from_utf8_lossy(&payload);
                        if args.timestamps {
                            println!("{} {}", timestamp, payload_str);
                        } else {
                            println!("{}", payload_str);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
