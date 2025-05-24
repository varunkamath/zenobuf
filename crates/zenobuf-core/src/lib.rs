//! Zenobuf Core - A simpler ROS-like framework in Rust
//!
//! This crate provides the core functionality for the Zenobuf framework,
//! a simpler ROS-like framework in Rust that uses Zenoh for transport
//! and Protocol Buffers for serialization.

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
pub use node::{Node, PublisherHandle, SubscriberHandle, ServiceHandle, ClientHandle, DropGuard};
pub use parameter::Parameter;
pub use publisher::Publisher;
pub use qos::{QosProfile, QosPreset};
pub use service::Service;
pub use subscriber::Subscriber;
pub use transport::{ZenohTransport, Transport};
