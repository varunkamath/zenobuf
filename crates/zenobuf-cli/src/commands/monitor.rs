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

    /// Exit after this many seconds
    #[clap(short = 'T', long)]
    timeout: Option<u64>,
}

/// Executes the monitor command
pub async fn execute(args: MonitorArgs) -> Result<()> {
    println!(
        "{label} {topic}",
        label = style("Monitoring topic:").bold(),
        topic = args.topic
    );
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

    // Optional timeout: sleep for the given duration, or pend forever if unset
    let timeout_fut = async {
        match args.timeout {
            Some(secs) => tokio::time::sleep(std::time::Duration::from_secs(secs)).await,
            None => std::future::pending().await,
        }
    };
    pin!(timeout_fut);

    // Process messages until Ctrl+C or timeout
    loop {
        tokio::select! {
            _ = &mut interrupt => {
                println!("\nMonitoring stopped");
                break;
            }
            _ = &mut timeout_fut => {
                println!("\nMonitoring timed out");
                break;
            }
            sample = stream.next() => {
                if let Some(sample) = sample {
                    let payload = sample.payload().to_bytes();

                    let display = if args.json {
                        serde_json::from_slice::<Value>(&payload)
                            .ok()
                            .and_then(|json| serde_json::to_string_pretty(&json).ok())
                            .unwrap_or_else(|| String::from_utf8_lossy(&payload).into_owned())
                    } else {
                        String::from_utf8_lossy(&payload).into_owned()
                    };

                    if args.timestamps {
                        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
                        println!("{timestamp} {display}");
                    } else {
                        println!("{display}");
                    }
                }
            }
        }
    }

    Ok(())
}
