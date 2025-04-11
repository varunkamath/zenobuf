use std::sync::{Arc, Mutex};

use prost::Message as ProstMessage;
use zenobuf_core::message::Message;
use zenobuf_core::node::Node;
use zenobuf_core::qos::QosProfile;
use zenobuf_core::transport::ZenohTransport;

// Define a test message
#[derive(Clone, PartialEq, Debug, Default)]
struct TestMessage {
    value: i32,
    text: String,
}

// Implement ProstMessage for TestMessage
impl ProstMessage for TestMessage {
    fn encode(&self, buf: &mut impl prost::bytes::BufMut) -> Result<(), prost::EncodeError> {
        // Simple encoding for testing
        buf.put_slice(&self.value.to_le_bytes());
        buf.put_slice(&(self.text.len() as u32).to_le_bytes());
        buf.put_slice(self.text.as_bytes());
        Ok(())
    }

    fn decode(buf: impl prost::bytes::Buf) -> Result<Self, prost::DecodeError> {
        let mut buf = buf;
        if buf.remaining() < 8 {
            return Err(prost::DecodeError::new("Buffer too short"));
        }

        let mut bytes = [0u8; 4];
        buf.copy_to_slice(&mut bytes);
        let value = i32::from_le_bytes(bytes);

        buf.copy_to_slice(&mut bytes);
        let text_len = u32::from_le_bytes(bytes) as usize;

        if buf.remaining() < text_len {
            return Err(prost::DecodeError::new("Buffer too short for text"));
        }

        let mut text_bytes = vec![0u8; text_len];
        buf.copy_to_slice(&mut text_bytes);
        let text = String::from_utf8_lossy(&text_bytes).to_string();

        Ok(TestMessage { value, text })
    }

    fn encoded_len(&self) -> usize {
        8 + self.text.len() // 4 bytes for value, 4 bytes for text length, plus text
    }

    fn clear(&mut self) {
        self.value = 0;
        self.text.clear();
    }

    fn merge_field(
        &mut self,
        _tag: u32,
        _wire_type: prost::encoding::WireType,
        _buf: &mut impl prost::bytes::Buf,
        _ctx: prost::encoding::DecodeContext,
    ) -> Result<(), prost::DecodeError> {
        // Not needed for our tests
        Ok(())
    }

    fn encode_raw(&self, _buf: &mut impl prost::bytes::BufMut) {
        // Not needed for our tests
    }
}

// Implement Message for TestMessage
impl Message for TestMessage {
    fn type_name() -> &'static str {
        "TestMessage"
    }
}

// Define a test request message
#[derive(Clone, PartialEq, Debug, Default)]
struct AddRequest {
    a: i32,
    b: i32,
}

// Implement ProstMessage for AddRequest
impl ProstMessage for AddRequest {
    fn encode(&self, buf: &mut impl prost::bytes::BufMut) -> Result<(), prost::EncodeError> {
        buf.put_slice(&self.a.to_le_bytes());
        buf.put_slice(&self.b.to_le_bytes());
        Ok(())
    }

    fn decode(buf: impl prost::bytes::Buf) -> Result<Self, prost::DecodeError> {
        let mut buf = buf;
        if buf.remaining() < 8 {
            return Err(prost::DecodeError::new("Buffer too short"));
        }

        let mut bytes = [0u8; 4];
        buf.copy_to_slice(&mut bytes);
        let a = i32::from_le_bytes(bytes);

        buf.copy_to_slice(&mut bytes);
        let b = i32::from_le_bytes(bytes);

        Ok(AddRequest { a, b })
    }

    fn encoded_len(&self) -> usize {
        8 // 4 bytes for a, 4 bytes for b
    }

    fn clear(&mut self) {
        self.a = 0;
        self.b = 0;
    }

    fn merge_field(
        &mut self,
        _tag: u32,
        _wire_type: prost::encoding::WireType,
        _buf: &mut impl prost::bytes::Buf,
        _ctx: prost::encoding::DecodeContext,
    ) -> Result<(), prost::DecodeError> {
        // Not needed for our tests
        Ok(())
    }

    fn encode_raw(&self, _buf: &mut impl prost::bytes::BufMut) {
        // Not needed for our tests
    }
}

// Implement Message for AddRequest
impl Message for AddRequest {
    fn type_name() -> &'static str {
        "AddRequest"
    }
}

// Define a test response message
#[derive(Clone, PartialEq, Debug, Default)]
struct AddResponse {
    sum: i32,
}

// Implement ProstMessage for AddResponse
impl ProstMessage for AddResponse {
    fn encode(&self, buf: &mut impl prost::bytes::BufMut) -> Result<(), prost::EncodeError> {
        buf.put_slice(&self.sum.to_le_bytes());
        Ok(())
    }

    fn decode(buf: impl prost::bytes::Buf) -> Result<Self, prost::DecodeError> {
        let mut buf = buf;
        if buf.remaining() < 4 {
            return Err(prost::DecodeError::new("Buffer too short"));
        }

        let mut bytes = [0u8; 4];
        buf.copy_to_slice(&mut bytes);
        let sum = i32::from_le_bytes(bytes);

        Ok(AddResponse { sum })
    }

    fn encoded_len(&self) -> usize {
        4 // 4 bytes for sum
    }

    fn clear(&mut self) {
        self.sum = 0;
    }

    fn merge_field(
        &mut self,
        _tag: u32,
        _wire_type: prost::encoding::WireType,
        _buf: &mut impl prost::bytes::Buf,
        _ctx: prost::encoding::DecodeContext,
    ) -> Result<(), prost::DecodeError> {
        // Not needed for our tests
        Ok(())
    }

    fn encode_raw(&self, _buf: &mut impl prost::bytes::BufMut) {
        // Not needed for our tests
    }
}

// Implement Message for AddResponse
impl Message for AddResponse {
    fn type_name() -> &'static str {
        "AddResponse"
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_parameter_basic() {
    // Create a transport
    let transport = ZenohTransport::new().await.unwrap();

    // Create a node
    let node = Node::with_transport("parameter_node", transport).unwrap();

    // Set parameters of different types
    node.set_parameter("string_param", "hello".to_string())
        .unwrap();
    node.set_parameter("int_param", 42).unwrap();
    node.set_parameter("float_param", std::f64::consts::PI)
        .unwrap();
    node.set_parameter("bool_param", true).unwrap();
    node.set_parameter("array_param", vec![1, 2, 3]).unwrap();

    // Get parameters
    let string_param = node.get_parameter::<String>("string_param").unwrap();
    let int_param = node.get_parameter::<i32>("int_param").unwrap();
    let float_param = node.get_parameter::<f64>("float_param").unwrap();
    let bool_param = node.get_parameter::<bool>("bool_param").unwrap();
    let array_param = node.get_parameter::<Vec<i32>>("array_param").unwrap();

    // Check parameters
    assert_eq!(string_param, "hello");
    assert_eq!(int_param, 42);
    assert_eq!(float_param, std::f64::consts::PI);
    assert!(bool_param);
    assert_eq!(array_param, vec![1, 2, 3]);

    // Try to get a non-existent parameter
    let result = node.get_parameter::<i32>("nonexistent");
    assert!(result.is_err());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_service_client_basic() {
    // Create a transport
    let transport = ZenohTransport::new().await.unwrap();

    // Create a node
    let node = Node::with_transport("service_client_node", transport).unwrap();

    // Create a service
    let _service = node
        .create_service::<AddRequest, AddResponse, _>("add_service", |req: AddRequest| {
            Ok(AddResponse { sum: req.a + req.b })
        })
        .await
        .unwrap();

    // Create a client
    let client = node
        .create_client::<AddRequest, AddResponse>("add_service")
        .unwrap();

    // Create a request
    let request = AddRequest { a: 40, b: 2 };

    // Call the service
    let response = client.call(&request).unwrap();

    // Check the response
    assert_eq!(response.sum, 42);

    // Call the service asynchronously
    let response = client.call_async(&request).await.unwrap();

    // Check the response
    assert_eq!(response.sum, 42);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_pub_sub_basic() {
    // Create a transport
    let transport = ZenohTransport::new().await.unwrap();

    // Create a node
    let node = Node::with_transport("pub_sub_node", transport).unwrap();

    // Create a variable to store the received message
    let received = Arc::new(Mutex::new(None));
    let received_clone = received.clone();

    // Create a subscriber
    let _subscriber = node
        .create_subscriber::<TestMessage, _>(
            "test_topic",
            QosProfile::default(),
            move |msg: TestMessage| {
                let mut received = received_clone.lock().unwrap();
                *received = Some(msg);
            },
        )
        .await
        .unwrap();

    // Create a publisher
    let publisher = node
        .create_publisher::<TestMessage>("test_topic", QosProfile::default())
        .await
        .unwrap();

    // Create a message to publish
    let message = TestMessage {
        value: 42,
        text: "Hello, world!".to_string(),
    };

    // Publish the message
    publisher.publish(&message).unwrap();

    // Spin once to process callbacks
    node.spin_once().unwrap();

    // Check that the message was received
    let received_msg = received.lock().unwrap();
    assert!(received_msg.is_some());
    assert_eq!(received_msg.as_ref().unwrap().value, 42);
    assert_eq!(received_msg.as_ref().unwrap().text, "Hello, world!");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_complex_workflow() {
    // Create a transport
    let transport = ZenohTransport::new().await.unwrap();

    // Create a node
    let node = Node::with_transport("complex_node", transport).unwrap();

    // Set a parameter
    node.set_parameter("count", 0).unwrap();

    // Create a publisher
    let publisher = node
        .create_publisher::<TestMessage>("test_topic", QosProfile::default())
        .await
        .unwrap();

    // Create a service
    let _service = node
        .create_service::<AddRequest, AddResponse, _>("add_service", |req: AddRequest| {
            Ok(AddResponse { sum: req.a + req.b })
        })
        .await
        .unwrap();

    // Create a client
    let client = node
        .create_client::<AddRequest, AddResponse>("add_service")
        .unwrap();

    // Create a variable to store the received message
    let received = Arc::new(Mutex::new(None));
    let received_clone = received.clone();

    // Create a subscriber
    let _subscriber = node
        .create_subscriber::<TestMessage, _>(
            "test_topic",
            QosProfile::default(),
            move |msg: TestMessage| {
                let mut received = received_clone.lock().unwrap();
                *received = Some(msg);
            },
        )
        .await
        .unwrap();

    // Publish a message
    let message = TestMessage {
        value: 42,
        text: "Hello, world!".to_string(),
    };
    publisher.publish(&message).unwrap();

    // Call the service
    let request = AddRequest { a: 40, b: 2 };
    let response = client.call(&request).unwrap();

    // Check the response
    assert_eq!(response.sum, 42);

    // Spin once to process callbacks
    node.spin_once().unwrap();

    // Check that the message was received
    let received_msg = received.lock().unwrap();
    assert!(received_msg.is_some());
    assert_eq!(received_msg.as_ref().unwrap().value, 42);
    assert_eq!(received_msg.as_ref().unwrap().text, "Hello, world!");

    // Update the parameter
    node.set_parameter("count", 1).unwrap();

    // Check the parameter
    let count = node.get_parameter::<i32>("count").unwrap();
    assert_eq!(count, 1);
}
