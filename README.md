# Zenobuf

A simpler ROS-like framework in Rust, using Zenoh for transport and Protocol Buffers for serialization.

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

Generate Rust code from your `.proto` files using `prost-build` in your `build.rs`:

```rust
fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["protos/my_messages.proto"], &["protos"])?;
    Ok(())
}
```

Implement the `Message` trait for your generated types:

```rust
use zenobuf_macros::ZenobufMessage;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/my_messages.rs"));
    
    impl zenobuf_core::Message for Point {
        fn type_name() -> &'static str {
            "my_messages.Point"
        }
    }
}
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
let publisher = node.create_publisher::<Point>("point_topic", QosProfile::default())?;

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
let _subscriber = node.create_subscriber::<Point>(
    "point_topic",
    QosProfile::default(),
    |point| {
        println!("Received point: ({}, {}, {})", point.x, point.y, point.z);
    },
)?;

// Spin the node
node.spin().await?;
```

#### Creating a Service

```rust
use zenobuf_core::{Node, Result};
use my_crate::proto::{AddRequest, AddResponse};

// Create a service
let _service = node.create_service::<AddRequest, AddResponse, _>(
    "add_service",
    |request| {
        let response = AddResponse {
            sum: request.a + request.b,
        };
        Ok(response)
    },
)?;

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

## Examples

See the `zenobuf-examples` directory for complete examples:

- `talker.rs`: Simple publisher example
- `listener.rs`: Simple subscriber example
- `service.rs`: Service example
- `client.rs`: Client example
- `parameters.rs`: Parameter example
- `complete_app.rs`: Complete application example

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
