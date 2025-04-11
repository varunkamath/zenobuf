# Zenobuf Implementation Plan

## Overview

Zenobuf is a simpler ROS-like framework in Rust that provides a stripped-back and ergonomic library implementation. It uses Zenoh for the transport layer and Protocol Buffers for serialization.

## Core Principles

1. **Simplicity**: Provide a minimal but complete API that covers the core functionality of ROS
2. **Ergonomics**: Create an API that feels natural to Rust developers
3. **Type Safety**: Leverage Rust's type system for compile-time guarantees
4. **Performance**: Utilize Zenoh's efficient pub/sub system and Protocol Buffers' serialization
5. **User-Defined Messages**: Support user-defined Protocol Buffer messages without requiring a code generation step in the framework

## Project Structure

The project will be organized as a Cargo workspace with multiple crates:

```
zenobuf/
├── Cargo.toml (workspace)
├── zenobuf-core/
├── zenobuf-macros/
├── zenobuf-cli/
└── zenobuf-examples/
```

### Crate Responsibilities

1. **zenobuf-core**: Core library with the basic abstractions
2. **zenobuf-macros**: Procedural macros for integrating user-defined Protocol Buffer messages
3. **zenobuf-cli**: Command-line tools for node discovery, topic monitoring, etc.
4. **zenobuf-examples**: Example applications demonstrating the framework's usage

## Detailed Implementation Plan

### Phase 1: Core Infrastructure

#### 1.1 Set up the workspace structure

- Create the Cargo workspace
- Set up the basic directory structure
- Configure dependencies
- Set up CI/CD with GitHub Actions

#### 1.2 Implement the Message trait system

- Define the `Message` trait that user-defined Protocol Buffer messages will implement
- Create utilities for working with Protocol Buffer messages
- Ensure compatibility with user-provided Protocol Buffer messages

#### 1.3 Implement the transport layer with Zenoh

- Create the `ZenohTransport` abstraction
- Implement session management
- Set up error handling
- Create serialization/deserialization utilities

### Phase 2: Core Abstractions

#### 2.1 Implement the Node abstraction

- Create the `Node` struct
- Implement node lifecycle management
- Set up the spinning mechanism
- Implement node discovery

#### 2.2 Implement Publishers and Subscribers

- Create the `Publisher` and `Subscriber` structs
- Implement the creation methods on `Node`
- Set up the callback mechanisms
- Implement QoS profiles

#### 2.3 Implement Services and Clients

- Create the `Service` and `Client` structs
- Implement the creation methods on `Node`
- Set up the request/response pattern
- Implement service discovery

### Phase 3: Additional Features

#### 3.1 Implement the Parameter system

- Create the `Parameter` abstraction
- Implement parameter storage using Zenoh
- Set up parameter change notifications
- Implement parameter services

#### 3.2 Create the CLI tools

- Implement node discovery
- Implement topic monitoring
- Implement service introspection
- Implement parameter management

#### 3.3 Create example applications

- Simple publisher/subscriber examples
- Service/client examples
- Parameter examples
- Complete application examples

### Phase 4: Documentation and Polish

#### 4.1 Write comprehensive documentation

- API documentation
- User guide
- Tutorials
- Migration guide for ROS users

#### 4.2 Performance optimization

- Benchmark the framework
- Optimize critical paths
- Reduce allocations

#### 4.3 Final testing and release

- End-to-end testing
- Compatibility testing
- Release v0.1.0

## Detailed File Structure

```
zenobuf/
├── Cargo.toml
├── .gitignore
├── README.md
├── zenobuf-core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── node.rs
│       ├── publisher.rs
│       ├── subscriber.rs
│       ├── service.rs
│       ├── client.rs
│       ├── message.rs
│       ├── transport/
│       │   ├── mod.rs
│       │   └── zenoh.rs
│       ├── error.rs
│       ├── qos.rs
│       ├── parameter.rs
│       ├── time.rs
│       └── util.rs
├── zenobuf-macros/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── message.rs
├── zenobuf-cli/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── commands/
│           ├── mod.rs
│           ├── list.rs
│           ├── monitor.rs
│           ├── call.rs
│           └── param.rs
└── zenobuf-examples/
    ├── Cargo.toml
    ├── protos/
    │   ├── geometry.proto
    │   └── example_service.proto
    └── src/
        ├── bin/
        │   ├── talker.rs
        │   ├── listener.rs
        │   ├── service.rs
        │   ├── client.rs
        │   ├── parameters.rs
        │   └── complete_app.rs
        └── lib.rs
```

## Key Components

### Message System

The message system will be designed to work with user-provided Protocol Buffer messages:

1. Users define their messages in `.proto` files
2. Users use `prost` or another Protocol Buffer library to generate Rust code
3. Users implement the `zenobuf::Message` trait for their message types (or use our derive macro)
4. The framework uses these message types for pub/sub and services

```rust
// User-defined message
#[derive(Clone, PartialEq, ::prost::Message, ZenobufMessage)]
pub struct Point {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
    #[prost(float, tag = "3")]
    pub z: f32,
}

// Our trait
pub trait Message: prost::Message + Default + Clone + Send + Sync + 'static {
    fn type_name() -> &'static str;
}
```

### Node System

The Node system will be the central abstraction for users:

```rust
// Creating a node
let mut node = Node::new("my_node")?;

// Spinning a node (processing callbacks)
node.spin().await; // Async version
node.spin_once(); // Process one batch of callbacks
```

### Publisher/Subscriber System

The pub/sub system will be type-safe and easy to use:

```rust
// Creating a publisher
let publisher = node.create_publisher::<MyMessage>("topic_name", QosProfile::default())?;

// Publishing a message
publisher.publish(&my_message)?;

// Creating a subscriber with a callback
let subscriber = node.create_subscriber::<MyMessage>(
    "topic_name",
    QosProfile::default(),
    |msg| {
        println!("Received: {:?}", msg);
    },
)?;

// Or with async
let subscriber = node.create_subscriber_async::<MyMessage>(
    "topic_name",
    QosProfile::default(),
)?;

while let Some(msg) = subscriber.next().await {
    println!("Received: {:?}", msg);
}
```

### Service/Client System

The service/client system will follow a request/response pattern:

```rust
// Creating a service
let service = node.create_service::<AddTwoInts>(
    "add_two_ints",
    |request| {
        let response = AddTwoIntsResponse {
            sum: request.a + request.b,
        };
        Ok(response)
    },
)?;

// Creating a client
let client = node.create_client::<AddTwoInts>("add_two_ints")?;

// Calling a service
let response = client.call(&request).await?;
```

### Parameter System

The parameter system will provide a way to store and retrieve configuration values:

```rust
// Setting a parameter
node.set_parameter("my_param", 42)?;

// Getting a parameter
let value: i32 = node.get_parameter("my_param")?;

// Parameter change callback
node.on_parameter_change("my_param", |value: i32| {
    println!("Parameter changed to: {}", value);
})?;
```

## Dependencies

### zenobuf-core

```toml
[dependencies]
zenoh = "1.3.3"
prost = "0.13.5"
async-trait = "0.1"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
thiserror = "1"
tracing = "0.1"
```

### zenobuf-macros

```toml
[dependencies]
proc-macro2 = "1"
quote = "1"
syn = { version = "2", features = ["full"] }
```

### zenobuf-cli

```toml
[dependencies]
zenobuf-core = { path = "../zenobuf-core" }
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
console = "0.15"
indicatif = "0.17"
```

### zenobuf-examples

```toml
[dependencies]
zenobuf-core = { path = "../zenobuf-core" }
zenobuf-macros = { path = "../zenobuf-macros" }
prost = "0.13.5"
tokio = { version = "1", features = ["full"] }

[build-dependencies]
prost-build = "0.13.5"
```

## Timeline

1. **Phase 1 (Core Infrastructure)**: 2-3 weeks
2. **Phase 2 (Core Abstractions)**: 3-4 weeks
3. **Phase 3 (Additional Features)**: 2-3 weeks
4. **Phase 4 (Documentation and Polish)**: 2 weeks

Total estimated time: 9-12 weeks

## Challenges and Considerations

1. **Message Compatibility**: Ensuring that user-defined Protocol Buffer messages work seamlessly with the framework
2. **Zenoh Integration**: Properly integrating with Zenoh's API, which may change in future versions
3. **Performance**: Ensuring that the framework has minimal overhead compared to using Zenoh directly
4. **Error Handling**: Creating a comprehensive error handling system that is both informative and ergonomic
5. **Async/Sync API**: Providing both synchronous and asynchronous APIs where appropriate
6. **Documentation**: Creating comprehensive documentation that is accessible to both Rust developers and ROS users

## Success Criteria

1. **Functionality**: The framework provides all the core functionality of ROS (pub/sub, services, parameters)
2. **Ergonomics**: The API is intuitive and easy to use for Rust developers
3. **Performance**: The framework has minimal overhead compared to using Zenoh directly
4. **Documentation**: The framework has comprehensive documentation and examples
5. **Adoption**: The framework is adopted by the Rust robotics community
