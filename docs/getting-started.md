# Getting Started with Zenobuf

Zenobuf is a lightweight, ergonomic framework for building distributed systems in Rust. This guide will walk you through creating your first Zenobuf application step by step.

## What is Zenobuf?

Zenobuf provides a ROS-like framework with:
- **Publish-Subscribe Messaging**: Send and receive messages on topics
- **Service-Based RPC**: Request-response communication between nodes
- **Parameter System**: Store and retrieve configuration values
- **Type-Safe API**: Leverage Rust's type system for compile-time guarantees
- **Protocol Buffers Integration**: Use Protocol Buffers for message serialization
- **Zenoh Transport**: Efficient pub/sub and query/reply using Zenoh

## Prerequisites

Before you begin, make sure you have:
- **Rust 1.70 or later**: Install from [rustup.rs](https://rustup.rs/)
- **Protocol Buffers compiler (`protoc`) 3.0 or later**: 
  - On Ubuntu/Debian: `sudo apt install protobuf-compiler`
  - On macOS: `brew install protobuf`
  - On Windows: Download from [Protocol Buffers releases](https://github.com/protocolbuffers/protobuf/releases)

## Quick Start: Using the Starter Template

The fastest way to get started is to use our starter template:

```bash
# Clone the repository
git clone https://github.com/varunkamath/zenobuf
cd zenobuf

# Copy the starter template
cp -r starter-template my-zenobuf-app
cd my-zenobuf-app

# Update dependencies to use published crates
# Edit Cargo.toml and change path dependencies to version dependencies:
# zenobuf-core = "0.2"
# zenobuf-macros = "0.2"

# Run the example
cargo run
```

This template includes a complete working example with publishers, subscribers, services, and clients.

## Step-by-Step Tutorial

Let's build a simple distributed system from scratch.

### Step 1: Create a New Project

```bash
cargo new my-zenobuf-app
cd my-zenobuf-app
```

### Step 2: Add Dependencies

Edit your `Cargo.toml`:

```toml
[package]
name = "my-zenobuf-app"
version = "0.1.0"
edition = "2021"

[dependencies]
zenobuf-core = "0.2"
zenobuf-macros = "0.2"
prost = "0.13"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"

[build-dependencies]
prost-build = "0.13"
```

### Step 3: Define Your Messages

Create a `protos` directory and add your Protocol Buffer definitions:

```bash
mkdir protos
```

Create `protos/messages.proto`:

```protobuf
syntax = "proto3";

package my_app;

// A simple point in 3D space
message Point {
  float x = 1;
  float y = 2;
  float z = 3;
}

// A sensor reading
message SensorReading {
  string sensor_id = 1;
  double value = 2;
  int64 timestamp = 3;
}

// Service messages
message GetStatusRequest {
  string node_name = 1;
}

message GetStatusResponse {
  string status = 1;
  int32 uptime_seconds = 2;
}
```

### Step 4: Setup Build Script

Create `build.rs` in your project root:

```rust
fn main() -> std::io::Result<()> {
    // Compile Protocol Buffer definitions with automatic ZenobufMessage derive
    prost_build::Config::new()
        .type_attribute(".", "#[derive(zenobuf_macros::ZenobufMessage)]")
        .compile_protos(&["protos/messages.proto"], &["protos"])?;
    Ok(())
}
```

### Step 5: Create Your First Publisher

Replace the contents of `src/main.rs`:

```rust
use std::time::Duration;
use zenobuf_core::{Node, QosProfile};
use tracing_subscriber;

// Include the generated Protocol Buffer code
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/my_app.rs"));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create a node
    let node = Node::new("sensor_publisher").await?;

    // Create a publisher for sensor readings
    let sensor_publisher = node
        .publisher::<proto::SensorReading>("sensor_data")
        .with_qos(QosProfile::default())
        .build()
        .await?;

    // Create a publisher for points
    let point_publisher = node
        .publisher::<proto::Point>("points")
        .build()
        .await?;

    println!("📡 Publisher started! Publishing sensor data and points...");

    // Publish messages in a loop
    let mut counter = 0;
    loop {
        // Create a sensor reading
        let reading = proto::SensorReading {
            sensor_id: "temp_sensor_01".to_string(),
            value: 20.0 + (counter as f64 * 0.1),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs() as i64,
        };

        // Create a point
        let point = proto::Point {
            x: (counter as f32).sin(),
            y: (counter as f32).cos(),
            z: counter as f32 * 0.1,
        };

        // Publish the messages
        sensor_publisher.publish(&reading)?;
        point_publisher.publish(&point)?;

        println!("📤 Published sensor reading: {} = {:.2}°C", 
                reading.sensor_id, reading.value);
        println!("📤 Published point: ({:.2}, {:.2}, {:.2})", 
                point.x, point.y, point.z);

        // Wait before next publication
        tokio::time::sleep(Duration::from_secs(2)).await;
        counter += 1;
    }
}
```

### Step 6: Test Your Publisher

Run your publisher:

```bash
cargo run
```

You should see output like:
```
📡 Publisher started! Publishing sensor data and points...
📤 Published sensor reading: temp_sensor_01 = 20.00°C
📤 Published point: (0.00, 1.00, 0.00)
📤 Published sensor reading: temp_sensor_01 = 20.10°C
📤 Published point: (0.84, 0.54, 0.10)
...
```

### Step 7: Create a Subscriber

Create `src/bin/subscriber.rs`:

```rust
use zenobuf_core::{Node, QosProfile};
use tracing_subscriber;

// Include the generated Protocol Buffer code
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/my_app.rs"));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create a node
    let node = Node::new("sensor_subscriber").await?;

    // Create a subscriber for sensor readings
    let _sensor_subscriber = node
        .subscriber::<proto::SensorReading>("sensor_data")
        .with_qos(QosProfile::default())
        .build(|reading| {
            println!("🌡️  Received sensor data: {} = {:.2}°C at timestamp {}", 
                    reading.sensor_id, reading.value, reading.timestamp);
        })
        .await?;

    // Create a subscriber for points
    let _point_subscriber = node
        .subscriber::<proto::Point>("points")
        .build(|point| {
            println!("📍 Received point: ({:.2}, {:.2}, {:.2})", 
                    point.x, point.y, point.z);
        })
        .await?;

    println!("👂 Subscriber started! Listening for messages...");

    // Keep the node running
    node.spin().await?;

    Ok(())
}
```

Update your `Cargo.toml` to include the binary:

```toml
[[bin]]
name = "subscriber"
path = "src/bin/subscriber.rs"
```

### Step 8: Test Publisher and Subscriber

Run the subscriber in one terminal:

```bash
cargo run --bin subscriber
```

Run the publisher in another terminal:

```bash
cargo run
```

You should see the subscriber receiving messages from the publisher!

### Step 9: Add a Service

Create `src/bin/service.rs`:

```rust
use zenobuf_core::{Node, Result};
use tracing_subscriber;

// Include the generated Protocol Buffer code
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/my_app.rs"));
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create a node
    let node = Node::new("status_service").await?;

    // Track service start time
    let start_time = std::time::Instant::now();

    // Create a service
    let _service = node
        .service::<proto::GetStatusRequest, proto::GetStatusResponse>("get_status")
        .build(move |request| {
            println!("🔧 Received status request for: {}", request.node_name);

            let uptime = start_time.elapsed().as_secs() as i32;
            let response = proto::GetStatusResponse {
                status: "healthy".to_string(),
                uptime_seconds: uptime,
            };

            println!("📋 Sending status: {} (uptime: {}s)", response.status, response.uptime_seconds);
            Ok(response)
        })
        .await?;

    println!("🔧 Status service ready!");

    // Keep the service running
    node.spin().await?;

    Ok(())
}
```

### Step 10: Create a Client

Create `src/bin/client.rs`:

```rust
use std::time::Duration;
use zenobuf_core::{Node, Result};
use tracing_subscriber;

// Include the generated Protocol Buffer code
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/my_app.rs"));
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create a node
    let node = Node::new("status_client").await?;

    // Create a client
    let client = node
        .client::<proto::GetStatusRequest, proto::GetStatusResponse>("get_status")
        .build()?;

    println!("📞 Client ready! Calling service every 5 seconds...");

    // Call the service periodically
    loop {
        let request = proto::GetStatusRequest {
            node_name: "status_service".to_string(),
        };

        match client.call(&request) {
            Ok(response) => {
                println!("✅ Service response: {} (uptime: {}s)", 
                        response.status, response.uptime_seconds);
            }
            Err(e) => {
                println!("❌ Service call failed: {}", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
```

Update your `Cargo.toml`:

```toml
[[bin]]
name = "subscriber"
path = "src/bin/subscriber.rs"

[[bin]]
name = "service"
path = "src/bin/service.rs"

[[bin]]
name = "client"
path = "src/bin/client.rs"
```

### Step 11: Test the Complete System

Now you can run all components:

1. **Terminal 1 - Service**: `cargo run --bin service`
2. **Terminal 2 - Subscriber**: `cargo run --bin subscriber`
3. **Terminal 3 - Publisher**: `cargo run`
4. **Terminal 4 - Client**: `cargo run --bin client`

You now have a complete distributed system with pub/sub messaging and RPC services!

## Working with Parameters

Zenobuf also supports a parameter system for configuration:

```rust
use zenobuf_core::Node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::new("param_example").await?;

    // Set parameters
    node.set_parameter("max_speed", 10.0)?;
    node.set_parameter("robot_name", "rover_01".to_string())?;
    node.set_parameter("debug_mode", true)?;

    // Get parameters
    let max_speed: f64 = node.get_parameter("max_speed")?;
    let robot_name: String = node.get_parameter("robot_name")?;
    let debug_mode: bool = node.get_parameter("debug_mode")?;

    println!("Max speed: {}", max_speed);
    println!("Robot name: {}", robot_name);
    println!("Debug mode: {}", debug_mode);

    Ok(())
}
```

## Quality of Service (QoS)

Zenobuf supports QoS profiles for reliable communication:

```rust
use zenobuf_core::{Node, QosProfile, QosPreset};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::new("qos_example").await?;

    // Use a preset QoS profile
    let publisher = node
        .publisher::<proto::Point>("critical_data")
        .with_qos(QosProfile::from_preset(QosPreset::Reliable))
        .build()
        .await?;

    // Or create a custom QoS profile
    let custom_qos = QosProfile::default()
        .with_reliability(true)
        .with_durability(true);

    let subscriber = node
        .subscriber::<proto::Point>("critical_data")
        .with_qos(custom_qos)
        .build(|point| {
            println!("Received critical point: {:?}", point);
        })
        .await?;

    Ok(())
}
```

## Command Line Tools

Zenobuf includes CLI tools for monitoring and debugging:

```bash
# Install the CLI tools
cargo install zenobuf-cli

# List active nodes
zenobuf-cli list nodes

# List active topics
zenobuf-cli list topics

# Monitor a topic
zenobuf-cli monitor sensor_data

# Call a service
zenobuf-cli call get_status --data '{"node_name": "my_node"}'

# Get a parameter
zenobuf-cli param get max_speed

# Set a parameter
zenobuf-cli param set max_speed 15.0
```

## Next Steps

Now that you have a working Zenobuf application, you can:

1. **Explore the Examples**: Check out the `zenobuf-examples` crate for more complex examples
2. **Read the API Guide**: See `docs/api-guide.md` for detailed API documentation
3. **Learn the Architecture**: Read `docs/architecture.md` to understand how Zenobuf works
4. **Build Your Application**: Start building your own distributed system!

## Common Patterns

### Error Handling

```rust
use zenobuf_core::{Node, Result, Error};

#[tokio::main]
async fn main() -> Result<()> {
    let node = Node::new("error_example").await?;

    // Handle specific errors
    match node.get_parameter::<String>("missing_param") {
        Ok(value) => println!("Parameter value: {}", value),
        Err(Error::Parameter { .. }) => {
            println!("Parameter not found, using default");
            node.set_parameter("missing_param", "default_value".to_string())?;
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
```

### Graceful Shutdown

```rust
use tokio::signal;
use zenobuf_core::Node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::new("graceful_shutdown").await?;

    // Setup publishers, subscribers, etc.
    let _publisher = node.publisher::<proto::Point>("points").build().await?;

    // Wait for Ctrl+C
    signal::ctrl_c().await?;
    println!("Shutting down gracefully...");

    // Cleanup happens automatically when node is dropped
    Ok(())
}
```

### Multiple Nodes in One Process

```rust
use zenobuf_core::Node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create multiple nodes
    let publisher_node = Node::new("publisher").await?;
    let subscriber_node = Node::new("subscriber").await?;

    // Setup publisher on first node
    let publisher = publisher_node
        .publisher::<proto::Point>("points")
        .build()
        .await?;

    // Setup subscriber on second node
    let _subscriber = subscriber_node
        .subscriber::<proto::Point>("points")
        .build(|point| {
            println!("Received: {:?}", point);
        })
        .await?;

    // Both nodes can run concurrently
    tokio::try_join!(
        publisher_node.spin(),
        subscriber_node.spin()
    )?;

    Ok(())
}
```

## Troubleshooting

### Common Issues

1. **"protoc not found"**: Install the Protocol Buffers compiler
2. **"ZenobufMessage not implemented"**: Make sure you have the derive macro in your build.rs
3. **"No subscribers found"**: Publishers and subscribers need time to discover each other
4. **"Service not found"**: Make sure the service is running before calling it

### Debug Tips

1. Enable debug logging: `RUST_LOG=debug cargo run`
2. Use the CLI tools to inspect the system: `zenobuf-cli list topics`
3. Check that all nodes are using the same message types
4. Verify your Protocol Buffer definitions are correct

## Getting Help

- **Documentation**: [docs.rs/zenobuf-core](https://docs.rs/zenobuf-core)
- **Examples**: Check the `zenobuf-examples` crate
- **Issues**: [GitHub Issues](https://github.com/varunkamath/zenobuf/issues)
- **Discussions**: [GitHub Discussions](https://github.com/varunkamath/zenobuf/discussions)

Happy coding with Zenobuf! 🚀
