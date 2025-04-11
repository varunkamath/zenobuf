//! Zenobuf Core - A simpler ROS-like framework in Rust
//!
//! This crate provides the core functionality for the Zenobuf framework,
//! a simpler ROS-like framework in Rust that uses Zenoh for transport
//! and Protocol Buffers for serialization.

pub mod error;
pub mod message;
pub mod node;
pub mod publisher;
pub mod subscriber;
pub mod service;
pub mod client;
pub mod transport;
pub mod qos;
pub mod parameter;
pub mod time;
pub mod util;

// Re-export key types
pub use error::{Error, Result};
pub use message::Message;
pub use node::Node;
pub use publisher::Publisher;
pub use subscriber::Subscriber;
pub use service::Service;
pub use client::Client;
pub use qos::QosProfile;
pub use parameter::Parameter;
