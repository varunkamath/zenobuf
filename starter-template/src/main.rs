use zenobuf_core::Node;

// Include your generated protobuf messages
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/my_app.rs"));
}

use proto::{Point, AddRequest, AddResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Zenobuf example app...");

    // Create a node
    let node = Node::new("my_app").await?;
    println!("✅ Node created");

    // Create a publisher
    let publisher = node
        .publisher::<Point>("points")
        .build()
        .await?;
    println!("✅ Publisher created");

    // Create a subscriber
    let _subscriber = node
        .subscriber::<Point>("points")
        .build(|point| {
            println!("📨 Received point: ({}, {}, {})", point.x, point.y, point.z);
        })
        .await?;
    println!("✅ Subscriber created");

    // Create a service
    let _service = node
        .service::<AddRequest, AddResponse>("add")
        .build(|request| {
            println!("🔧 Service: Adding {} + {}", request.a, request.b);
            Ok(AddResponse {
                sum: request.a + request.b,
            })
        })
        .await?;
    println!("✅ Service created");

    // Create a client
    let client = node
        .client::<AddRequest, AddResponse>("add")
        .build()?;
    println!("✅ Client created");

    // Publish some messages and call services
    println!("\n🎯 Publishing messages and calling services...");
    for i in 0..3 {
        // Publish a point
        let point = Point {
            x: i as f32,
            y: (i * 2) as f32,
            z: (i * 3) as f32,
        };
        publisher.publish(&point)?;
        println!("📤 Published point #{}", i);

        // Call the service
        let response = client.call(&AddRequest { a: i, b: i + 1 })?;
        println!("🔄 Service response: {} + {} = {}", i, i + 1, response.sum);

        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    // Keep running for a bit to see messages
    println!("\n⏳ Waiting to see all messages...");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    println!("✨ Example completed successfully!");
    Ok(())
}
