//! Example client for the Zenobuf framework

use std::env;

use zenobuf_core::{Node, Result};
use zenobuf_examples::proto::service::{AddTwoIntsRequest, AddTwoIntsResponse};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <a> <b>", args[0]);
        std::process::exit(1);
    }

    let a = args[1].parse::<i32>().expect("Failed to parse a");
    let b = args[2].parse::<i32>().expect("Failed to parse b");

    // Create a node
    let node = Node::new("add_two_ints_client").await?;

    // Create a client
    let client = node.create_client::<AddTwoIntsRequest, AddTwoIntsResponse>("add_two_ints")?;

    // Create a request
    let request = AddTwoIntsRequest { a, b };

    // Call the service
    println!("Calling service with {} + {}", a, b);
    let response = client.call(&request)?;

    // Print the response
    println!("Service response: {} + {} = {}", a, b, response.sum);

    Ok(())
}
