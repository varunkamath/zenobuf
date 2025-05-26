# Zenobuf API Reference Guide

This guide provides comprehensive documentation for the Zenobuf API, including detailed examples and usage patterns.

## Table of Contents

- [Core Concepts](#core-concepts)
- [Node API](#node-api)
- [Publisher API](#publisher-api)
- [Subscriber API](#subscriber-api)
- [Service API](#service-api)
- [Client API](#client-api)
- [Message Trait](#message-trait)
- [Quality of Service (QoS)](#quality-of-service-qos)
- [Parameter System](#parameter-system)
- [Error Handling](#error-handling)
- [Advanced Usage](#advanced-usage)

## Core Concepts

### Node

A `Node` is the main entry point for Zenobuf applications. It manages publishers, subscribers, services, and clients.

```rust
use zenobuf_core::Node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::new("my_node").await?;
    // Use the node...
    Ok(())
}
```

### Message

The `Message` trait defines types that can be sent over the network. Use the `ZenobufMessage` derive macro for Protocol Buffer types.

```rust
use zenobuf_macros::ZenobufMessage;

#[derive(Clone, PartialEq, Default, ZenobufMessage)]
pub struct MyMessage {
    pub value: i32,
}
```

### Topics and Services

- **Topics**: Named channels for publish-subscribe messaging
- **Services**: Named endpoints for request-response communication

## Node API

### Creating a Node

```rust
use zenobuf_core::Node;

// Create a node with a unique name
let node = Node::new("unique_node_name").await?;
```

**Important**: Node names must be unique within the system.

### Node Methods

#### Publishers

```rust
// Create a publisher builder
let publisher_builder = node.publisher::<MessageType>("topic_name");

// Build with default settings
let publisher = publisher_builder.build().await?;

// Build with custom QoS
let publisher = publisher_builder
    .with_qos(QosProfile::default())
    .build()
    .await?;
```

#### Subscribers

```rust
// Create a subscriber with a callback
let subscriber = node
    .subscriber::<MessageType>("topic_name")
    .build(|message| {
        println!("Received: {:?}", message);
    })
    .await?;

// With custom QoS
let subscriber = node
    .subscriber::<MessageType>("topic_name")
    .with_qos(QosProfile::reliable())
    .build(|message| {
        // Handle message
    })
    .await?;
```

#### Services

```rust
// Create a service
let service = node
    .service::<RequestType, ResponseType>("service_name")
    .build(|request| {
        // Process request and return response
        Ok(ResponseType { /* ... */ })
    })
    .await?;
```

#### Clients

```rust
// Create a client
let client = node
    .client::<RequestType, ResponseType>("service_name")
    .build()?;
```

#### Parameters

```rust
// Set a parameter
node.set_parameter("param_name", value)?;

// Get a parameter
let value: Type = node.get_parameter("param_name")?;
```

#### Node Lifecycle

```rust
// Keep the node running (blocks until shutdown)
node.spin().await?;

// The node automatically cleans up when dropped
```

## Publisher API

### Creating Publishers

```rust
use zenobuf_core::{Node, QosProfile};

let node = Node::new("publisher_node").await?;

// Basic publisher
let publisher = node
    .publisher::<MyMessage>("my_topic")
    .build()
    .await?;

// Publisher with QoS
let publisher = node
    .publisher::<MyMessage>("my_topic")
    .with_qos(QosProfile::reliable())
    .build()
    .await?;
```

### Publishing Messages

```rust
// Create a message
let message = MyMessage { value: 42 };

// Publish the message
publisher.publish(&message)?;

// Publishers are thread-safe and can be cloned
let publisher_clone = publisher.clone();
tokio::spawn(async move {
    let msg = MyMessage { value: 100 };
    publisher_clone.publish(&msg).unwrap();
});
```

### Publisher Methods

```rust
impl<M: Message> Publisher<M> {
    /// Publish a message
    pub fn publish(&self, message: &M) -> Result<()>;
    
    /// Get the topic name
    pub fn topic(&self) -> &str;
    
    /// Get the QoS profile
    pub fn qos(&self) -> &QosProfile;
}
```

### Publisher Examples

#### Periodic Publishing

```rust
use std::time::Duration;
use tokio::time::interval;

let mut timer = interval(Duration::from_secs(1));
let mut counter = 0;

loop {
    timer.tick().await;
    
    let message = MyMessage { value: counter };
    publisher.publish(&message)?;
    
    counter += 1;
}
```

#### Conditional Publishing

```rust
fn should_publish(data: &SensorData) -> bool {
    data.value > 100.0 || data.is_critical
}

if should_publish(&sensor_data) {
    let message = SensorMessage::from(sensor_data);
    publisher.publish(&message)?;
}
```

## Subscriber API

### Creating Subscribers

```rust
use zenobuf_core::{Node, QosProfile};

let node = Node::new("subscriber_node").await?;

// Basic subscriber
let subscriber = node
    .subscriber::<MyMessage>("my_topic")
    .build(|message| {
        println!("Received: {:?}", message);
    })
    .await?;

// Subscriber with QoS
let subscriber = node
    .subscriber::<MyMessage>("my_topic")
    .with_qos(QosProfile::reliable())
    .build(|message| {
        // Handle message
    })
    .await?;
```

### Subscriber Callbacks

Subscriber callbacks must be `Fn(M) + Send + Sync + 'static`:

```rust
// Simple callback
let subscriber = node
    .subscriber::<MyMessage>("topic")
    .build(|msg| println!("Got: {}", msg.value))
    .await?;

// Callback with shared state
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let counter_clone = counter.clone();

let subscriber = node
    .subscriber::<MyMessage>("topic")
    .build(move |msg| {
        let mut count = counter_clone.lock().unwrap();
        *count += 1;
        println!("Message #{}: {}", *count, msg.value);
    })
    .await?;

// Async processing (spawn a task)
let subscriber = node
    .subscriber::<MyMessage>("topic")
    .build(|msg| {
        tokio::spawn(async move {
            // Async processing
            process_message_async(msg).await;
        });
    })
    .await?;
```

### Subscriber Examples

#### Message Filtering

```rust
let subscriber = node
    .subscriber::<SensorReading>("sensors")
    .build(|reading| {
        if reading.sensor_id.starts_with("temp_") {
            println!("Temperature: {}°C", reading.value);
        }
    })
    .await?;
```

#### Message Aggregation

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

let readings = Arc::new(Mutex::new(HashMap::new()));
let readings_clone = readings.clone();

let subscriber = node
    .subscriber::<SensorReading>("sensors")
    .build(move |reading| {
        let mut map = readings_clone.lock().unwrap();
        map.insert(reading.sensor_id.clone(), reading.value);
        
        if map.len() >= 5 {
            println!("Aggregated readings: {:?}", *map);
            map.clear();
        }
    })
    .await?;
```

## Service API

### Creating Services

```rust
use zenobuf_core::{Node, Result};

let node = Node::new("service_node").await?;

// Basic service
let service = node
    .service::<AddRequest, AddResponse>("add_service")
    .build(|request| {
        Ok(AddResponse {
            sum: request.a + request.b,
        })
    })
    .await?;
```

### Service Handlers

Service handlers must return `Result<ResponseType>`:

```rust
// Simple handler
let service = node
    .service::<MathRequest, MathResponse>("math")
    .build(|req| {
        let result = match req.operation.as_str() {
            "add" => req.a + req.b,
            "multiply" => req.a * req.b,
            _ => return Err(Error::Other { 
                reason: "Unknown operation".to_string() 
            }),
        };
        
        Ok(MathResponse { result })
    })
    .await?;

// Handler with shared state
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let counter_clone = counter.clone();

let service = node
    .service::<CountRequest, CountResponse>("counter")
    .build(move |_req| {
        let mut count = counter_clone.lock().unwrap();
        *count += 1;
        Ok(CountResponse { count: *count })
    })
    .await?;

// Async handler (spawn a task for heavy work)
let service = node
    .service::<ProcessRequest, ProcessResponse>("processor")
    .build(|req| {
        // For CPU-intensive work, use spawn_blocking
        let result = tokio::task::spawn_blocking(move || {
            heavy_computation(req.data)
        }).await.map_err(|e| Error::Other { 
            reason: format!("Task failed: {}", e) 
        })?;
        
        Ok(ProcessResponse { result })
    })
    .await?;
```

### Service Examples

#### Database Service

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Database = Arc<Mutex<HashMap<String, String>>>;

let db: Database = Arc::new(Mutex::new(HashMap::new()));

// Get service
let db_clone = db.clone();
let get_service = node
    .service::<GetRequest, GetResponse>("db_get")
    .build(move |req| {
        let db = db_clone.lock().unwrap();
        match db.get(&req.key) {
            Some(value) => Ok(GetResponse { 
                value: value.clone(),
                found: true,
            }),
            None => Ok(GetResponse {
                value: String::new(),
                found: false,
            }),
        }
    })
    .await?;

// Set service
let db_clone = db.clone();
let set_service = node
    .service::<SetRequest, SetResponse>("db_set")
    .build(move |req| {
        let mut db = db_clone.lock().unwrap();
        db.insert(req.key, req.value);
        Ok(SetResponse { success: true })
    })
    .await?;
```

#### File Service

```rust
use tokio::fs;

let file_service = node
    .service::<ReadFileRequest, ReadFileResponse>("read_file")
    .build(|req| async move {
        match fs::read_to_string(&req.path).await {
            Ok(content) => Ok(ReadFileResponse {
                content,
                success: true,
                error: String::new(),
            }),
            Err(e) => Ok(ReadFileResponse {
                content: String::new(),
                success: false,
                error: e.to_string(),
            }),
        }
    })
    .await?;
```

## Client API

### Creating Clients

```rust
let node = Node::new("client_node").await?;

let client = node
    .client::<RequestType, ResponseType>("service_name")
    .build()?;
```

### Making Service Calls

```rust
// Synchronous call
let request = AddRequest { a: 5, b: 3 };
let response = client.call(&request)?;
println!("Result: {}", response.sum);

// Asynchronous call
let request = AddRequest { a: 10, b: 20 };
let response = client.call_async(&request).await?;
println!("Async result: {}", response.sum);
```

### Client Methods

```rust
impl<Req: Message, Res: Message> Client<Req, Res> {
    /// Make a synchronous service call
    pub fn call(&self, request: &Req) -> Result<Res>;
    
    /// Make an asynchronous service call
    pub async fn call_async(&self, request: &Req) -> Result<Res>;
    
    /// Get the service name
    pub fn name(&self) -> &str;
}
```

### Client Examples

#### Retry Logic

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn call_with_retry<Req, Res>(
    client: &Client<Req, Res>,
    request: &Req,
    max_retries: u32,
) -> Result<Res>
where
    Req: Message,
    Res: Message,
{
    for attempt in 0..=max_retries {
        match client.call_async(request).await {
            Ok(response) => return Ok(response),
            Err(e) if attempt < max_retries => {
                println!("Attempt {} failed: {}, retrying...", attempt + 1, e);
                sleep(Duration::from_millis(100 * (attempt + 1) as u64)).await;
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}

// Usage
let request = MyRequest { data: "test".to_string() };
let response = call_with_retry(&client, &request, 3).await?;
```

#### Parallel Calls

```rust
use futures::future::try_join_all;

// Make multiple calls in parallel
let requests = vec![
    AddRequest { a: 1, b: 2 },
    AddRequest { a: 3, b: 4 },
    AddRequest { a: 5, b: 6 },
];

let futures = requests.iter().map(|req| client.call_async(req));
let responses = try_join_all(futures).await?;

for (i, response) in responses.iter().enumerate() {
    println!("Result {}: {}", i, response.sum);
}
```

#### Client Pool

```rust
use std::sync::Arc;

#[derive(Clone)]
struct ClientPool<Req: Message, Res: Message> {
    clients: Vec<Arc<Client<Req, Res>>>,
    current: Arc<std::sync::atomic::AtomicUsize>,
}

impl<Req: Message, Res: Message> ClientPool<Req, Res> {
    fn new(clients: Vec<Client<Req, Res>>) -> Self {
        Self {
            clients: clients.into_iter().map(Arc::new).collect(),
            current: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }
    
    fn get_client(&self) -> &Arc<Client<Req, Res>> {
        let index = self.current.fetch_add(1, std::sync::atomic::Ordering::Relaxed) 
                    % self.clients.len();
        &self.clients[index]
    }
    
    async fn call(&self, request: &Req) -> Result<Res> {
        self.get_client().call_async(request).await
    }
}
```

## Message Trait

### Implementing Message

The `Message` trait is automatically implemented when you use the `ZenobufMessage` derive macro:

```rust
use zenobuf_macros::ZenobufMessage;

#[derive(Clone, PartialEq, Default, ZenobufMessage)]
pub struct MyMessage {
    pub field1: String,
    pub field2: i32,
}
```

### Message Requirements

Types that implement `Message` must also implement:
- `Clone`
- `PartialEq`
- `Default`
- `Send + Sync + 'static`
- `prost::Message` (for Protocol Buffer types)

### Custom Message Implementation

You can manually implement the `Message` trait:

```rust
use zenobuf_core::Message;
use prost::Message as ProstMessage;

#[derive(Clone, PartialEq, Default)]
pub struct CustomMessage {
    pub data: Vec<u8>,
}

impl Message for CustomMessage {
    fn type_name() -> &'static str {
        "CustomMessage"
    }
}

// You also need to implement prost::Message
impl ProstMessage for CustomMessage {
    // Implementation details...
}
```

## Quality of Service (QoS)

### QoS Profiles

```rust
use zenobuf_core::{QosProfile, QosPreset};

// Use preset profiles
let reliable_qos = QosProfile::from_preset(QosPreset::Reliable);
let best_effort_qos = QosProfile::from_preset(QosPreset::BestEffort);

// Create custom profiles
let custom_qos = QosProfile::default()
    .with_reliability(true)
    .with_durability(true);
```

### QoS Settings

```rust
impl QosProfile {
    /// Set reliability (guaranteed delivery)
    pub fn with_reliability(self, reliable: bool) -> Self;
    
    /// Set durability (late-joining subscribers get messages)
    pub fn with_durability(self, durable: bool) -> Self;
    
    /// Create from preset
    pub fn from_preset(preset: QosPreset) -> Self;
}
```

### Using QoS

```rust
// Publisher with QoS
let publisher = node
    .publisher::<MyMessage>("topic")
    .with_qos(QosProfile::reliable())
    .build()
    .await?;

// Subscriber with QoS
let subscriber = node
    .subscriber::<MyMessage>("topic")
    .with_qos(QosProfile::reliable())
    .build(|msg| { /* handle */ })
    .await?;
```

## Parameter System

### Setting Parameters

```rust
// Set different types of parameters
node.set_parameter("string_param", "hello".to_string())?;
node.set_parameter("int_param", 42i32)?;
node.set_parameter("float_param", 3.14f64)?;
node.set_parameter("bool_param", true)?;

// Set complex types (must be serializable)
#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    max_speed: f64,
    debug: bool,
}

let config = Config { max_speed: 10.0, debug: true };
node.set_parameter("config", config)?;
```

### Getting Parameters

```rust
// Get parameters with type inference
let string_val: String = node.get_parameter("string_param")?;
let int_val: i32 = node.get_parameter("int_param")?;
let float_val: f64 = node.get_parameter("float_param")?;
let bool_val: bool = node.get_parameter("bool_param")?;

// Get complex types
let config: Config = node.get_parameter("config")?;
```

### Parameter Examples

#### Configuration Management

```rust
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct RobotConfig {
    max_speed: f64,
    sensor_rate: u32,
    debug_mode: bool,
    waypoints: Vec<(f64, f64)>,
}

impl Default for RobotConfig {
    fn default() -> Self {
        Self {
            max_speed: 1.0,
            sensor_rate: 10,
            debug_mode: false,
            waypoints: vec![(0.0, 0.0)],
        }
    }
}

// Load configuration
let config = node.get_parameter::<RobotConfig>("robot_config")
    .unwrap_or_else(|_| {
        let default_config = RobotConfig::default();
        node.set_parameter("robot_config", default_config.clone()).unwrap();
        default_config
    });
```

#### Dynamic Reconfiguration

```rust
use std::sync::{Arc, Mutex};
use std::time::Duration;

let config = Arc::new(Mutex::new(RobotConfig::default()));
let config_clone = config.clone();

// Periodically check for parameter updates
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    
    loop {
        interval.tick().await;
        
        if let Ok(new_config) = node.get_parameter::<RobotConfig>("robot_config") {
            let mut current_config = config_clone.lock().unwrap();
            if *current_config != new_config {
                println!("Configuration updated!");
                *current_config = new_config;
            }
        }
    }
});
```

## Error Handling

### Error Types

```rust
use zenobuf_core::{Error, Result};

// Zenobuf uses a comprehensive error type
match some_operation() {
    Ok(result) => { /* handle success */ },
    Err(Error::Transport { source, context }) => {
        println!("Transport error in {}: {}", context, source);
    },
    Err(Error::MessageSerialization { type_name, .. }) => {
        println!("Failed to serialize {}", type_name);
    },
    Err(Error::ServiceCallTimeout { service, .. }) => {
        println!("Service {} timed out", service);
    },
    Err(e) => {
        println!("Other error: {}", e);
    },
}
```

### Error Context

```rust
use zenobuf_core::ErrorContext;

// Add context to errors
let result = some_operation()
    .with_context("while processing sensor data")?;

// Add formatted context
let result = some_operation()
    .with_context_f(|| format!("processing sensor {}", sensor_id))?;
```

### Custom Error Handling

```rust
// Create custom error types
fn validate_message(msg: &MyMessage) -> Result<()> {
    if msg.value < 0 {
        return Err(Error::Other {
            reason: "Value cannot be negative".to_string(),
        });
    }
    Ok(())
}

// Use in publishers
let message = MyMessage { value: -1 };
validate_message(&message)?;
publisher.publish(&message)?;
```

## Advanced Usage

### Multiple Nodes

```rust
// Create multiple nodes in the same process
let node1 = Node::new("publisher_node").await?;
let node2 = Node::new("subscriber_node").await?;

// Setup communication between nodes
let publisher = node1.publisher::<MyMessage>("topic").build().await?;
let _subscriber = node2
    .subscriber::<MyMessage>("topic")
    .build(|msg| println!("Received: {:?}", msg))
    .await?;

// Run both nodes concurrently
tokio::try_join!(
    node1.spin(),
    node2.spin()
)?;
```

### Resource Management

```rust
// Publishers, subscribers, services, and clients are automatically cleaned up
// when dropped, but you can also explicitly manage them:

let publisher_handle = node.publisher::<MyMessage>("topic").build().await?;
let subscriber_handle = node
    .subscriber::<MyMessage>("topic")
    .build(|msg| { /* handle */ })
    .await?;

// Handles implement Drop for automatic cleanup
drop(publisher_handle);
drop(subscriber_handle);
```

### Custom Transport

```rust
// Zenobuf uses Zenoh by default, but you can implement custom transports
use zenobuf_core::transport::Transport;

// This is advanced usage - most users should stick with the default Zenoh transport
```

### Performance Optimization

#### Message Pooling

```rust
use std::sync::Arc;

// For high-frequency publishing, consider message pooling
struct MessagePool<T> {
    pool: Arc<Mutex<Vec<T>>>,
}

impl<T: Default> MessagePool<T> {
    fn get(&self) -> T {
        self.pool.lock().unwrap().pop().unwrap_or_default()
    }
    
    fn return_message(&self, mut msg: T) {
        // Reset message to default state
        msg = T::default();
        self.pool.lock().unwrap().push(msg);
    }
}
```

#### Batch Processing

```rust
use std::collections::VecDeque;
use std::time::{Duration, Instant};

// Batch messages for efficient processing
struct MessageBatcher<T> {
    buffer: VecDeque<T>,
    last_flush: Instant,
    batch_size: usize,
    flush_interval: Duration,
}

impl<T> MessageBatcher<T> {
    fn add_message(&mut self, msg: T) -> Option<Vec<T>> {
        self.buffer.push_back(msg);
        
        if self.buffer.len() >= self.batch_size 
           || self.last_flush.elapsed() >= self.flush_interval {
            self.flush()
        } else {
            None
        }
    }
    
    fn flush(&mut self) -> Option<Vec<T>> {
        if self.buffer.is_empty() {
            return None;
        }
        
        let batch = self.buffer.drain(..).collect();
        self.last_flush = Instant::now();
        Some(batch)
    }
}
```

### Testing

#### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use zenobuf_core::Node;
    
    #[tokio::test]
    async fn test_publisher_subscriber() {
        let node = Node::new("test_node").await.unwrap();
        
        let publisher = node
            .publisher::<MyMessage>("test_topic")
            .build()
            .await
            .unwrap();
        
        let received = Arc::new(Mutex::new(Vec::new()));
        let received_clone = received.clone();
        
        let _subscriber = node
            .subscriber::<MyMessage>("test_topic")
            .build(move |msg| {
                received_clone.lock().unwrap().push(msg);
            })
            .await
            .unwrap();
        
        // Give time for discovery
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let message = MyMessage { value: 42 };
        publisher.publish(&message).unwrap();
        
        // Give time for message delivery
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let messages = received.lock().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].value, 42);
    }
}
```

#### Integration Testing

```rust
#[tokio::test]
async fn test_service_client() {
    let service_node = Node::new("service_test").await.unwrap();
    let client_node = Node::new("client_test").await.unwrap();
    
    // Setup service
    let _service = service_node
        .service::<AddRequest, AddResponse>("add")
        .build(|req| Ok(AddResponse { sum: req.a + req.b }))
        .await
        .unwrap();
    
    // Setup client
    let client = client_node
        .client::<AddRequest, AddResponse>("add")
        .build()
        .unwrap();
    
    // Give time for discovery
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Test the service
    let request = AddRequest { a: 5, b: 3 };
    let response = client.call(&request).unwrap();
    assert_eq!(response.sum, 8);
}
```

This API guide covers the complete Zenobuf API with practical examples. For more examples, see the `zenobuf-examples` crate and the getting started guide.
