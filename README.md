# Zenobuf

A simple ROS-like framework in Rust, using Zenoh for transport and Protocol Buffers for serialization.

## Overview

Zenobuf is a lightweight, ergonomic framework for building distributed systems in Rust. It provides a publish-subscribe messaging system, a service-based RPC system, and a parameter system, similar to ROS (Robot Operating System) but with a more Rust-idiomatic API.

## Features

- **Publish-Subscribe Messaging**: Send and receive messages on topics
- **Service-Based RPC**: Request-response communication between nodes
- **Parameter System**: Store and retrieve configuration values
- **Type-Safe API**: Leverage Rust's type system for compile-time guarantees
- **Protocol Buffers Integration**: Use Protocol Buffers for message serialization
- **Zenoh Transport**: Efficient pub/sub and query/reply using Zenoh

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Protocol Buffers compiler (`protoc`) 3.0 or later

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
zenobuf-core = "0.1.0"
zenobuf-macros = "0.1.0"
```

### Usage

#### Defining Messages

Define your messages using Protocol Buffers:

```protobuf
syntax = "proto3";

package my_messages;

message Point {
  float x = 1;
  float y = 2;
  float z = 3;
}
```

**Step 1:** Add dependencies to your `Cargo.toml`:

```toml
[dependencies]
zenobuf-core = "0.2"
zenobuf-macros = "0.2"
prost = "0.13"
tokio = { version = "1", features = ["full"] }

[build-dependencies]
prost-build = "0.13"
```

**Step 2:** Create a `build.rs` file that automatically adds the derive macro:

```rust
fn main() -> std::io::Result<()> {
    prost_build::Config::new()
        .type_attribute(".", "#[derive(zenobuf_macros::ZenobufMessage)]")
        .compile_protos(&["protos/my_messages.proto"], &["protos"])?;
    Ok(())
}
```

**Step 3:** Include the generated code in your `lib.rs` or `main.rs`:

```rust
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/my_messages.rs"));
}
```

**That's it!** The `ZenobufMessage` derive macro is automatically added to all your protobuf types.

### 🚀 Quick Start Template

For the fastest way to get started, copy the [starter template](starter-template/) which includes:
- Pre-configured `Cargo.toml` and `build.rs`
- Example protobuf definitions
- Complete working example with publisher, subscriber, service, and client
- Step-by-step customization guide

```bash
cp -r starter-template my-zenobuf-app
cd my-zenobuf-app
cargo run
```

#### Creating a Node

```rust
use zenobuf_core::Node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a node
    let node = Node::new("my_node").await?;

    // ...

    Ok(())
}
```

#### Publishing Messages

```rust
use zenobuf_core::{Node, QosProfile};
use my_crate::proto::Point;

// Create a publisher
let publisher = node
    .publisher::<Point>("point_topic")
    .build()
    .await?;

// Create a message
let point = Point {
    x: 1.0,
    y: 2.0,
    z: 3.0,
};

// Publish the message
publisher.publish(&point)?;
```

#### Subscribing to Messages

```rust
use zenobuf_core::{Node, QosProfile};
use my_crate::proto::Point;

// Create a subscriber
let _subscriber = node
    .subscriber::<Point>("point_topic")
    .build(|point| {
        println!("Received point: ({}, {}, {})", point.x, point.y, point.z);
    })
    .await?;

// Spin the node
node.spin().await?;
```

#### Creating a Service

```rust
use zenobuf_core::{Node, Result};
use my_crate::proto::{AddRequest, AddResponse};

// Create a service
let _service = node
    .service::<AddRequest, AddResponse>("add_service")
    .build(|request| {
        Ok(AddResponse {
            sum: request.a + request.b,
        })
    })
    .await?;

// Spin the node
node.spin().await?;
```

#### Calling a Service

```rust
use zenobuf_core::{Node, Result};
use my_crate::proto::{AddRequest, AddResponse};

// Create a client
let client = node.create_client::<AddRequest, AddResponse>("add_service")?;

// Create a request
let request = AddRequest {
    a: 2,
    b: 3,
};

// Call the service
let response = client.call(&request)?;
println!("Sum: {}", response.sum);
```

#### Using Parameters

```rust
use zenobuf_core::{Node, Result};

// Set a parameter
node.set_parameter("my_param", 42)?;

// Get a parameter
let value: i32 = node.get_parameter("my_param")?;
println!("Parameter value: {}", value);
```

## 📚 Documentation

### Quick Links
- **[📖 Getting Started Guide](docs/getting-started.md)** - Complete tutorial from installation to your first app
- **[🔧 API Reference](docs/api-guide.md)** - Comprehensive API documentation with examples
- **[🏗️ Architecture Guide](docs/architecture.md)** - System design and internal architecture
- **[📦 Crate Documentation](https://docs.rs/zenobuf-core)** - API docs on docs.rs

### Examples

See the [`zenobuf-examples`](crates/zenobuf-examples/) crate for complete examples:

- **[talker.rs](crates/zenobuf-examples/src/bin/talker.rs)** - Publisher example
- **[listener.rs](crates/zenobuf-examples/src/bin/listener.rs)** - Subscriber example
- **[service.rs](crates/zenobuf-examples/src/bin/service.rs)** - Service example
- **[client.rs](crates/zenobuf-examples/src/bin/client.rs)** - Client example
- **[parameters.rs](crates/zenobuf-examples/src/bin/parameters.rs)** - Parameter example
- **[complete_app.rs](crates/zenobuf-examples/src/bin/complete_app.rs)** - Complete application

### CLI Tools

Install the CLI tools for development and debugging:

```bash
cargo install zenobuf-cli

# Monitor topics
zenobuf-cli monitor sensor_data

# List system components
zenobuf-cli list topics
zenobuf-cli list services
zenobuf-cli list nodes

# Call services
zenobuf-cli call add_service --data '{"a": 5, "b": 3}'

# Manage parameters
zenobuf-cli param get max_speed
zenobuf-cli param set max_speed 15.0
```

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
