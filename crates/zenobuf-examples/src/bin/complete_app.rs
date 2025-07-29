//! Complete application example for the Zenobuf framework

use std::time::Duration;

use zenobuf_core::{Node, QosProfile, Result};
use zenobuf_examples::proto::geometry::{Point, Pose, Quaternion};
use zenobuf_examples::proto::service::{AddTwoIntsRequest, AddTwoIntsResponse};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a node
    let node = Node::new("complete_app").await?;

    // Create a publisher using the builder pattern
    let publisher = node
        .publisher::<Pose>("pose")
        .with_qos(QosProfile::default())
        .build()
        .await?;

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

    // Sleep to ensure the service is registered
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Create a client using the builder pattern
    let client = node
        .client::<AddTwoIntsRequest, AddTwoIntsResponse>("add_two_ints")
        .build()?;

    // Set parameters
    node.set_parameter("string_param", "hello".to_string())?;
    node.set_parameter("int_param", 42)?;

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

    // Main loop
    let mut counter = 0;
    loop {
        // Update the message
        if let Some(position) = &mut pose.position {
            position.x = counter as f32;
        }

        // Publish the message
        publisher.publish(&pose)?;
        println!("Published pose with x = {counter}");

        // Call the service
        let request = AddTwoIntsRequest {
            a: counter,
            b: counter + 1,
        };

        match client.call(&request) {
            Ok(response) => {
                println!(
                    "Service response: {} + {} = {}",
                    counter,
                    counter + 1,
                    response.sum
                );
            }
            Err(e) => {
                println!("Service call failed: {e}");
            }
        };

        // Get parameters
        let string_param: String = node.get_parameter("string_param")?;
        let int_param: i32 = node.get_parameter("int_param")?;

        println!("Parameters:");
        println!("  string_param: {string_param}");
        println!("  int_param: {int_param}");

        // Sleep for a while
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Increment the counter
        counter += 1;
    }
}
