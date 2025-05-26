# Zenobuf Architecture

This document describes the architecture and design principles of the Zenobuf framework.

## Overview

Zenobuf is a distributed systems framework inspired by ROS (Robot Operating System) but designed specifically for Rust. It provides a type-safe, ergonomic API for building distributed applications with publish-subscribe messaging, request-response services, and a parameter system.

## Design Principles

### 1. Type Safety
Zenobuf leverages Rust's type system to provide compile-time guarantees about message types and API usage. The `Message` trait ensures that only valid types can be sent over the network.

### 2. Zero-Copy Where Possible
The framework minimizes data copying by using efficient serialization and leveraging Zenoh's zero-copy capabilities where possible.

### 3. Async-First
All I/O operations are asynchronous by default, built on top of Tokio for high-performance concurrent operations.

### 4. Ergonomic API
The API is designed to be intuitive and easy to use, with builder patterns and sensible defaults.

### 5. Pluggable Transport
While Zenoh is the default transport, the architecture allows for pluggable transport implementations.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
├─────────────────────────────────────────────────────────────┤
│                     Zenobuf API                            │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐│
│  │  Node   │ │Publisher│ │Subscriber│ │ Service │ │ Client  ││
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘│
├─────────────────────────────────────────────────────────────┤
│                   Message Layer                            │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐│
│  │ Message Trait   │ │ Serialization   │ │ Type Registry   ││
│  └─────────────────┘ └─────────────────┘ └─────────────────┘│
├─────────────────────────────────────────────────────────────┤
│                  Transport Layer                           │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐│
│  │ Transport Trait │ │ Zenoh Transport │ │ QoS Profiles    ││
│  └─────────────────┘ └─────────────────┘ └─────────────────┘│
├─────────────────────────────────────────────────────────────┤
│                   Network Layer                            │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                    Zenoh                                ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### Node

The `Node` is the central component that manages all communication endpoints. It provides:

- **Resource Management**: Tracks and manages publishers, subscribers, services, and clients
- **Discovery**: Handles automatic discovery of other nodes and services
- **Lifecycle Management**: Manages the lifecycle of communication endpoints
- **Parameter Storage**: Provides a distributed parameter system

```rust
pub struct Node {
    name: String,
    transport: Arc<dyn Transport>,
    publishers: Arc<Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
    subscribers: Arc<Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
    services: Arc<Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
    clients: Arc<Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
    parameters: Arc<Mutex<HashMap<String, Parameter>>>,
}
```

### Transport Layer

The transport layer abstracts the underlying communication mechanism. The default implementation uses Zenoh, but the design allows for other transports.

```rust
#[async_trait]
pub trait Transport: Send + Sync + 'static {
    async fn create_publisher<M: Message>(&self, topic: &str) -> Result<Arc<Publisher<M>>>;
    async fn create_subscriber<M: Message, F>(&self, topic: &str, callback: F) -> Result<Arc<Subscriber>>
    where F: Fn(M) + Send + Sync + 'static;
    async fn create_service<Req: Message, Res: Message, F>(&self, service: &str, handler: F) -> Result<Arc<Service>>
    where F: Fn(Req) -> Result<Res> + Send + Sync + 'static;
    fn create_client<Req: Message, Res: Message>(&self, service: &str) -> Result<Arc<Client<Req, Res>>>;
}
```

### Message System

The message system provides type-safe serialization and deserialization:

```rust
pub trait Message: ProstMessage + Default + Clone + Send + Sync + 'static {
    fn type_name() -> &'static str;
}
```

Messages are serialized using Protocol Buffers for efficient, cross-language compatibility.

## Communication Patterns

### Publish-Subscribe

```
Publisher Node                    Subscriber Node
┌─────────────┐                  ┌─────────────┐
│ Publisher   │ ──── Topic ────► │ Subscriber  │
│             │                  │             │
│ publish()   │                  │ callback()  │
└─────────────┘                  └─────────────┘
```

- **Decoupled**: Publishers and subscribers don't need to know about each other
- **Many-to-Many**: Multiple publishers can send to multiple subscribers
- **Asynchronous**: Non-blocking message delivery

### Request-Response (Services)

```
Client Node                      Service Node
┌─────────────┐                  ┌─────────────┐
│ Client      │ ──── Request ──► │ Service     │
│             │                  │             │
│ call()      │ ◄─── Response ── │ handler()   │
└─────────────┘                  └─────────────┘
```

- **Synchronous**: Blocking call with response
- **One-to-One**: Direct communication between client and service
- **Reliable**: Built-in error handling and timeouts

### Parameters

```
Node A                           Node B
┌─────────────┐                  ┌─────────────┐
│ set_param() │ ──── Zenoh ────► │ get_param() │
│             │                  │             │
└─────────────┘                  └─────────────┘
```

- **Distributed**: Parameters are shared across all nodes
- **Persistent**: Parameters persist until explicitly changed
- **Type-Safe**: Parameters maintain their types

## Quality of Service (QoS)

Zenobuf supports QoS profiles to control message delivery characteristics:

```rust
pub struct QosProfile {
    pub reliability: bool,    // Guaranteed delivery
    pub durability: bool,     // Late-joining subscribers get messages
}
```

QoS profiles are implemented at the transport layer and affect:
- **Reliability**: Whether messages are guaranteed to be delivered
- **Durability**: Whether messages are stored for late-joining subscribers
- **Ordering**: Whether message ordering is preserved

## Threading Model

Zenobuf uses an async-first design built on Tokio:

```
┌─────────────────────────────────────────────────────────────┐
│                    Tokio Runtime                            │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐│
│ │   Node      │ │ Publishers  │ │ Subscribers │ │  Services   ││
│ │   Tasks     │ │   Tasks     │ │   Tasks     │ │   Tasks     ││
│ └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘│
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐│
│ │  Transport  │ │   Zenoh     │ │  Network    │ │   Timer     ││
│ │   Tasks     │ │   Tasks     │ │   Tasks     │ │   Tasks     ││
│ └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘│
└─────────────────────────────────────────────────────────────┘
```

- **Single Runtime**: All operations run on a shared Tokio runtime
- **Task-Based**: Each communication endpoint runs in its own task
- **Non-Blocking**: All I/O operations are non-blocking
- **Thread-Safe**: All public APIs are thread-safe

## Memory Management

### Resource Cleanup

Zenobuf uses RAII (Resource Acquisition Is Initialization) for automatic cleanup:

```rust
pub struct DropGuard {
    cleanup: Box<dyn FnOnce() + Send + Sync>,
}

impl Drop for DropGuard {
    fn drop(&mut self) {
        (self.cleanup)();
    }
}
```

When publishers, subscribers, services, or clients are dropped, they automatically:
- Close network connections
- Stop background tasks
- Clean up resources

### Reference Counting

Internal components use `Arc<T>` for shared ownership:
- Publishers and subscribers can be cloned and shared across threads
- Services and clients maintain shared state safely
- Transport layer is shared across all endpoints

## Error Handling

Zenobuf uses a comprehensive error type that provides context:

```rust
#[derive(Error, Debug)]
pub enum Error {
    #[error("Transport error in {context}")]
    Transport { source: zenoh::Error, context: String },
    
    #[error("Message serialization failed for type {type_name}")]
    MessageSerialization { source: prost::EncodeError, type_name: &'static str },
    
    #[error("Service call timed out for service {service}")]
    ServiceCallTimeout { service: String, timeout: Duration },
    
    // ... more error variants
}
```

Errors include:
- **Context**: Where the error occurred
- **Source**: The underlying cause
- **Type Information**: What type was involved
- **Structured Data**: Machine-readable error information

## Discovery and Naming

### Node Discovery

Nodes automatically discover each other through the Zenoh network:
1. When a node starts, it announces itself
2. Other nodes receive the announcement
3. Nodes maintain a registry of active nodes

### Topic and Service Discovery

Topics and services are discovered dynamically:
1. Publishers announce their topics
2. Services announce their endpoints
3. Subscribers and clients discover available endpoints
4. Connections are established automatically

### Naming Conventions

- **Node Names**: Must be unique within the system
- **Topic Names**: Hierarchical (e.g., `/sensors/temperature`)
- **Service Names**: Flat namespace (e.g., `get_status`)
- **Parameter Names**: Hierarchical (e.g., `/robot/max_speed`)

## Performance Characteristics

### Latency

- **Pub/Sub**: Low latency (~1ms for local communication)
- **Services**: Higher latency due to request-response pattern
- **Parameters**: Cached locally for fast access

### Throughput

- **Message Size**: Optimized for small to medium messages (< 1MB)
- **Frequency**: Can handle high-frequency publishing (> 1kHz)
- **Concurrent Connections**: Scales to hundreds of endpoints

### Memory Usage

- **Per Node**: ~1-10MB base memory usage
- **Per Endpoint**: ~1-100KB depending on configuration
- **Message Buffers**: Configurable based on QoS settings

## Security Considerations

### Transport Security

Zenoh provides built-in security features:
- **Authentication**: Node identity verification
- **Encryption**: TLS encryption for network traffic
- **Authorization**: Access control for topics and services

### Message Validation

- **Type Safety**: Compile-time type checking
- **Schema Validation**: Protocol Buffer schema enforcement
- **Size Limits**: Configurable message size limits

## Deployment Patterns

### Single Process

Multiple nodes can run in a single process:
```rust
let node1 = Node::new("publisher").await?;
let node2 = Node::new("subscriber").await?;

tokio::try_join!(
    node1.spin(),
    node2.spin()
)?;
```

### Multi-Process

Nodes can run in separate processes and discover each other automatically.

### Distributed

Nodes can run across multiple machines with Zenoh handling network communication.

### Container Deployment

Each node can run in its own container with service discovery through the container orchestrator.

## Comparison with ROS

| Feature | ROS | Zenobuf |
|---------|-----|---------|
| Language | C++/Python | Rust |
| Type Safety | Runtime | Compile-time |
| Memory Safety | Manual | Automatic |
| Async Support | Limited | Native |
| Transport | Custom | Zenoh |
| Serialization | Custom | Protocol Buffers |
| Discovery | roscore | Distributed |
| Performance | Good | Excellent |

## Future Enhancements

### Planned Features

1. **Message Recording**: Record and replay message streams
2. **Monitoring Tools**: Web-based monitoring dashboard
3. **Load Balancing**: Automatic load balancing for services
4. **Federation**: Connect multiple Zenobuf networks
5. **Schema Evolution**: Support for message schema evolution

### Extension Points

The architecture supports extensions through:
- **Custom Transports**: Implement the `Transport` trait
- **Custom Serialization**: Implement the `Message` trait
- **Middleware**: Intercept messages for logging, filtering, etc.
- **Custom QoS**: Extend QoS profiles for specific needs

## Conclusion

Zenobuf's architecture provides a solid foundation for building distributed systems in Rust. The combination of type safety, performance, and ergonomics makes it well-suited for robotics, IoT, and other distributed applications.

The modular design allows for customization and extension while maintaining a simple, consistent API. The async-first approach ensures high performance and scalability, while the comprehensive error handling provides robust operation in production environments.
