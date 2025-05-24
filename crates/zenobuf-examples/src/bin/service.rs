//! Example service for the Zenobuf framework

use zenobuf_core::{Node, Result};
use zenobuf_examples::proto::service::{AddTwoIntsRequest, AddTwoIntsResponse};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a node
    let node = Node::new("add_two_ints_server").await?;

    // Create a service using the builder pattern
    let _service = node
        .service::<AddTwoIntsRequest, AddTwoIntsResponse>("add_two_ints")
        .build(|request| {
            println!("Received request: {} + {}", request.a, request.b);

            let response = AddTwoIntsResponse {
                sum: request.a + request.b,
            };

            println!("Sending response: {}", response.sum);
            Ok(response)
        })
        .await?;

    println!("Service ready");

    // Spin the node
    node.spin().await?;

    Ok(())
}
