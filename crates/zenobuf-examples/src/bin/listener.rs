//! Example subscriber (listener) for the Zenobuf framework

use zenobuf_core::{Node, QosProfile};
use zenobuf_examples::proto::geometry::Pose;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a node
    let node = Node::new("listener").await?;

    // Create a subscriber using the builder pattern
    let _subscriber = node
        .subscriber::<Pose>("pose")
        .with_qos(QosProfile::default())
        .build(|pose| {
            println!("Received pose:");
            if let Some(position) = &pose.position {
                println!(
                    "  Position: ({}, {}, {})",
                    position.x, position.y, position.z
                );
            }
            if let Some(orientation) = &pose.orientation {
                println!(
                    "  Orientation: ({}, {}, {}, {})",
                    orientation.x, orientation.y, orientation.z, orientation.w
                );
            }
        })
        .await?;

    // Spin the node
    node.spin().await?;

    Ok(())
}
