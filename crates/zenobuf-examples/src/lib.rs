//! # Zenobuf Examples - Example applications for the Zenobuf framework
//!
//! This crate contains comprehensive examples demonstrating how to use the Zenobuf
//! framework for building distributed systems. Each example showcases different
//! aspects of the framework and provides practical, runnable code.
//!
//! ## Available Examples
//!
//! ### Basic Communication
//!
//! - **[talker.rs](src/bin/talker.rs)** - Publisher example that sends geometry messages
//! - **[listener.rs](src/bin/listener.rs)** - Subscriber example that receives geometry messages
//!
//! ### Service Communication
//!
//! - **[service.rs](src/bin/service.rs)** - Service example that provides addition functionality
//! - **[client.rs](src/bin/client.rs)** - Client example that calls the addition service
//!
//! ### Advanced Examples
//!
//! - **[complete_app.rs](src/bin/complete_app.rs)** - Complete application with pub/sub, services, and parameters
//! - **[parameters.rs](src/bin/parameters.rs)** - Parameter system usage example
//!
//! ## Running the Examples
//!
//! ### Publisher/Subscriber Example
//!
//! ```bash
//! # Terminal 1: Start the listener
//! cargo run --bin listener
//!
//! # Terminal 2: Start the talker
//! cargo run --bin talker
//! ```
//!
//! ### Service/Client Example
//!
//! ```bash
//! # Terminal 1: Start the service
//! cargo run --bin service
//!
//! # Terminal 2: Call the service
//! cargo run --bin client 5 3
//! ```
//!
//! ### Complete Application
//!
//! ```bash
//! # Run the complete example (includes everything)
//! cargo run --bin complete_app
//! ```
//!
//! ### Parameter Example
//!
//! ```bash
//! # Run the parameter example
//! cargo run --bin parameters
//! ```
//!
//! ## Message Types
//!
//! The examples use Protocol Buffer messages defined in the `protos/` directory:
//!
//! ### Geometry Messages (`protos/geometry.proto`)
//!
//! - **Point** - 3D point with x, y, z coordinates
//! - **Quaternion** - Orientation representation
//! - **Pose** - Position and orientation combined
//!
//! ### Service Messages (`protos/example_service.proto`)
//!
//! - **AddTwoIntsRequest** - Request with two integers to add
//! - **AddTwoIntsResponse** - Response with the sum
//!
//! ## Key Concepts Demonstrated
//!
//! ### 1. Node Creation
//!
//! ```rust,no_run
//! use zenobuf_core::Node;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let node = Node::new("my_node").await?;
//!     // Use the node...
//!     Ok(())
//! }
//! ```
//!
//! ### 2. Publishing Messages
//!
//! ```rust,no_run
//! # use zenobuf_core::Node;
//! # use zenobuf_examples::proto::geometry::Point;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let node = Node::new("publisher").await?;
//! let publisher = node
//!     .publisher::<Point>("points")
//!     .build()
//!     .await?;
//!
//! let point = Point { x: 1.0, y: 2.0, z: 3.0 };
//! publisher.publish(&point)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### 3. Subscribing to Messages
//!
//! ```rust,no_run
//! # use zenobuf_core::Node;
//! # use zenobuf_examples::proto::geometry::Point;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let node = Node::new("subscriber").await?;
//! let _subscriber = node
//!     .subscriber::<Point>("points")
//!     .build(|point| {
//!         println!("Received: ({}, {}, {})", point.x, point.y, point.z);
//!     })
//!     .await?;
//!
//! node.spin().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### 4. Creating Services
//!
//! ```rust,no_run
//! # use zenobuf_core::{Node, Result};
//! # use zenobuf_examples::proto::service::{AddTwoIntsRequest, AddTwoIntsResponse};
//! # async fn example() -> Result<()> {
//! # let node = Node::new("service").await?;
//! let _service = node
//!     .service::<AddTwoIntsRequest, AddTwoIntsResponse>("add")
//!     .build(|request| {
//!         Ok(AddTwoIntsResponse {
//!             sum: request.a + request.b,
//!         })
//!     })
//!     .await?;
//!
//! node.spin().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### 5. Calling Services
//!
//! ```rust,no_run
//! # use zenobuf_core::{Node, Result};
//! # use zenobuf_examples::proto::service::{AddTwoIntsRequest, AddTwoIntsResponse};
//! # async fn example() -> Result<()> {
//! # let node = Node::new("client").await?;
//! let client = node
//!     .client::<AddTwoIntsRequest, AddTwoIntsResponse>("add")
//!     .build()?;
//!
//! let request = AddTwoIntsRequest { a: 5, b: 3 };
//! let response = client.call(&request)?;
//! println!("Result: {}", response.sum);
//! # Ok(())
//! # }
//! ```
//!
//! ## Learning Path
//!
//! 1. **Start with Basic Examples**: Run `talker` and `listener` to understand pub/sub
//! 2. **Try Service Communication**: Run `service` and `client` to understand RPC
//! 3. **Explore Parameters**: Run `parameters` to see configuration management
//! 4. **Study Complete Application**: Run `complete_app` to see everything together
//! 5. **Build Your Own**: Use these examples as templates for your applications
//!
//! ## Tips for Development
//!
//! - **Use the CLI tools**: `zenobuf-cli list topics` to see active topics
//! - **Monitor messages**: `zenobuf-cli monitor <topic>` to debug message flow
//! - **Check services**: `zenobuf-cli list services` to see available services
//! - **Manage parameters**: `zenobuf-cli param get/set` for runtime configuration
//!
//! ## Next Steps
//!
//! After exploring these examples:
//! - Read the [Getting Started Guide](../../docs/getting-started.md)
//! - Check the [API Reference](../../docs/api-guide.md)
//! - Use the [Starter Template](../../starter-template/) for your own projects

// Include the generated Protocol Buffer code
pub mod proto {
    pub mod geometry {
        include!(concat!(env!("OUT_DIR"), "/zenobuf.examples.geometry.rs"));
    }

    pub mod service {
        include!(concat!(env!("OUT_DIR"), "/zenobuf.examples.service.rs"));
    }
}
