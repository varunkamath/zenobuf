//! Example publisher (talker) for the Zenobuf framework

use std::time::Duration;

use zenobuf_core::{Node, QosProfile};
use zenobuf_examples::proto::geometry::{Point, Pose, Quaternion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a node
    let node = Node::new("talker").await?;

    // Create a publisher
    let publisher = node
        .create_publisher::<Pose>("pose", QosProfile::default())
        .await?;

    // Create a message
    let mut pose = Pose {
        position: Some(Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        }),
        orientation: Some(Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }),
    };

    // Publish the message periodically
    let mut counter = 0;
    loop {
        // Update the message
        if let Some(position) = &mut pose.position {
            position.x = counter as f32;
        }

        // Publish the message
        publisher.publish(&pose)?;
        println!("Published pose with x = {}", counter);

        // Sleep for a while
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Increment the counter
        counter += 1;
    }
}
