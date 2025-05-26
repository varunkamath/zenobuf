//! # Zenobuf Core - A simpler ROS-like framework in Rust
//!
//! Zenobuf is a lightweight, ergonomic framework for building distributed systems in Rust.
//! It provides a publish-subscribe messaging system, a service-based RPC system, and a
//! parameter system, similar to ROS (Robot Operating System) but with a more Rust-idiomatic API.
//!
//! ## Features
//!
//! - **Publish-Subscribe Messaging**: Send and receive messages on topics
//! - **Service-Based RPC**: Request-response communication between nodes
//! - **Parameter System**: Store and retrieve configuration values
//! - **Type-Safe API**: Leverage Rust's type system for compile-time guarantees
//! - **Protocol Buffers Integration**: Use Protocol Buffers for message serialization
//! - **Zenoh Transport**: Efficient pub/sub and query/reply using Zenoh
//!
//! ## Quick Start
//!
//! ### 1. Add Dependencies
//!
//! ```toml
//! [dependencies]
//! zenobuf-core = "0.2"
//! zenobuf-macros = "0.2"
//! prost = "0.13"
//! tokio = { version = "1", features = ["full"] }
//!
//! [build-dependencies]
//! prost-build = "0.13"
//! ```
//!
//! ### 2. Define Messages with Protocol Buffers
//!
//! Create `protos/messages.proto`:
//! ```protobuf
//! syntax = "proto3";
//!
//! package my_app;
//!
//! message Point {
//!   float x = 1;
//!   float y = 2;
//!   float z = 3;
//! }
//! ```
//!
//! ### 3. Setup Build Script
//!
//! Create `build.rs`:
//! ```rust,ignore
//! fn main() -> std::io::Result<()> {
//!     prost_build::Config::new()
//!         .type_attribute(".", "#[derive(zenobuf_macros::ZenobufMessage)]")
//!         .compile_protos(&["protos/messages.proto"], &["protos"])?;
//!     Ok(())
//! }
//! ```
//!
//! ### 4. Create a Node and Publisher
//!
//! ```rust,ignore
//! use zenobuf_core::Node;
//!
//! // Include generated protobuf code
//! pub mod proto {
//!     include!(concat!(env!("OUT_DIR"), "/my_app.rs"));
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a node
//!     let node = Node::new("my_node").await?;
//!
//!     // Create a publisher
//!     let publisher = node
//!         .publisher::<proto::Point>("points")
//!         .build()
//!         .await?;
//!
//!     // Create and publish a message
//!     let point = proto::Point {
//!         x: 1.0,
//!         y: 2.0,
//!         z: 3.0,
//!     };
//!     publisher.publish(&point)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### 5. Create a Subscriber
//!
//! ```rust,ignore
//! use zenobuf_core::Node;
//!
//! // Use the same proto module from above
//! # pub mod proto {
//! #     include!(concat!(env!("OUT_DIR"), "/my_app.rs"));
//! # }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let node = Node::new("subscriber_node").await?;
//!
//!     // Create a subscriber with a callback
//!     let _subscriber = node
//!         .subscriber::<proto::Point>("points")
//!         .build(|point| {
//!             println!("Received point: ({}, {}, {})", point.x, point.y, point.z);
//!         })
//!         .await?;
//!
//!     // Keep the node running
//!     node.spin().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! Zenobuf is built around several key concepts:
//!
//! - **[Node]**: The main entry point that manages publishers, subscribers, services, and clients
//! - **[Publisher]**: Sends messages to topics
//! - **[Subscriber]**: Receives messages from topics
//! - **[Service]**: Handles request-response communication
//! - **[Client]**: Makes requests to services
//! - **[Message]**: Trait for types that can be sent over the network
//! - **[QosProfile]**: Quality of Service settings for reliable communication
//!
//! ## Examples
//!
//! For complete examples, see the `zenobuf-examples` crate which includes:
//! - Publisher/Subscriber examples
//! - Service/Client examples
//! - Parameter usage
//! - Complete applications
//!
//! ## Getting Started Template
//!
//! The fastest way to get started is to copy the starter template:
//! ```bash
//! git clone https://github.com/varunkamath/zenobuf
//! cp -r zenobuf/starter-template my-zenobuf-app
//! cd my-zenobuf-app
//! cargo run
//! ```

pub mod client;
pub mod error;
pub mod message;
pub mod node;
pub mod parameter;
pub mod publisher;
pub mod qos;
pub mod service;
pub mod subscriber;
pub mod time;
pub mod transport;

// Re-export key types
pub use client::Client;
pub use error::{Error, Result};
pub use message::Message;
pub use node::{ClientHandle, DropGuard, Node, PublisherHandle, ServiceHandle, SubscriberHandle};
pub use parameter::Parameter;
pub use publisher::Publisher;
pub use qos::{QosPreset, QosProfile};
pub use service::Service;
pub use subscriber::Subscriber;
pub use transport::{Transport, ZenohTransport};
