//! Tests for the new resource management features

use std::sync::{Arc, Mutex};
use prost::Message as ProstMessage;
use zenobuf_core::{Node, DropGuard};
use zenobuf_core::transport::ZenohTransport;
use zenobuf_core::message::Message;

// Test message type
#[derive(Clone, PartialEq, Debug, Default)]
struct TestMessage {
    value: i32,
    text: String,
}

// Implement ProstMessage for TestMessage
impl ProstMessage for TestMessage {
    fn encode(&self, buf: &mut impl prost::bytes::BufMut) -> std::result::Result<(), prost::EncodeError> {
        buf.put_slice(&self.value.to_le_bytes());
        buf.put_slice(&(self.text.len() as u32).to_le_bytes());
        buf.put_slice(self.text.as_bytes());
        Ok(())
    }

    fn decode(buf: impl prost::bytes::Buf) -> std::result::Result<Self, prost::DecodeError> {
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
    ) -> std::result::Result<(), prost::DecodeError> {
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
    fn encode(&self, buf: &mut impl prost::bytes::BufMut) -> std::result::Result<(), prost::EncodeError> {
        buf.put_slice(&self.a.to_le_bytes());
        buf.put_slice(&self.b.to_le_bytes());
        Ok(())
    }

    fn decode(buf: impl prost::bytes::Buf) -> std::result::Result<Self, prost::DecodeError> {
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
    ) -> std::result::Result<(), prost::DecodeError> {
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
    fn encode(&self, buf: &mut impl prost::bytes::BufMut) -> std::result::Result<(), prost::EncodeError> {
        buf.put_slice(&self.sum.to_le_bytes());
        Ok(())
    }

    fn decode(buf: impl prost::bytes::Buf) -> std::result::Result<Self, prost::DecodeError> {
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
    ) -> std::result::Result<(), prost::DecodeError> {
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
async fn test_publisher_handle_automatic_cleanup() {
    let transport = ZenohTransport::new().await.unwrap();
    let node = Node::with_transport("test_node", transport).unwrap();

    // Create a publisher handle
    let publisher_handle = node
        .publisher::<TestMessage>("test_topic")
        .build()
        .await
        .unwrap();

    // Test that we can publish messages
    let message = TestMessage {
        value: 42,
        text: "Hello, World!".to_string(),
    };

    publisher_handle.publish(&message).unwrap();
    assert_eq!(publisher_handle.topic(), "test_topic");

    // When the handle is dropped, cleanup should happen automatically
    drop(publisher_handle);

    // The test passes if no panics occur during cleanup
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_subscriber_handle_automatic_cleanup() {
    let transport = ZenohTransport::new().await.unwrap();
    let node = Node::with_transport("test_node", transport).unwrap();

    let received = Arc::new(Mutex::new(None));
    let received_clone = received.clone();

    // Create a subscriber handle
    let subscriber_handle = node
        .subscriber::<TestMessage>("test_topic")
        .build(move |msg: TestMessage| {
            let mut received = received_clone.lock().unwrap();
            *received = Some(msg);
        })
        .await
        .unwrap();

    // Test that the subscriber is created (just verify it exists)
    let _ = subscriber_handle.subscriber();

    // When the handle is dropped, cleanup should happen automatically
    drop(subscriber_handle);

    // The test passes if no panics occur during cleanup
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_service_handle_automatic_cleanup() {
    let transport = ZenohTransport::new().await.unwrap();
    let node = Node::with_transport("test_node", transport).unwrap();

    // Create a service handle
    let service_handle = node
        .service::<AddRequest, AddResponse>("add_service")
        .build(|req: AddRequest| {
            Ok(AddResponse { sum: req.a + req.b })
        })
        .await
        .unwrap();

    // Test that the service is created (just verify it exists)
    let _ = service_handle.service();

    // When the handle is dropped, cleanup should happen automatically
    drop(service_handle);

    // The test passes if no panics occur during cleanup
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_client_handle_automatic_cleanup() {
    let transport = ZenohTransport::new().await.unwrap();
    let node = Node::with_transport("test_node", transport).unwrap();

    // Create a client handle
    let client_handle = node
        .client::<AddRequest, AddResponse>("add_service")
        .build()
        .unwrap();

    // Test that the client is created (just verify it exists)
    let _ = client_handle.client();

    // When the handle is dropped, cleanup should happen automatically
    drop(client_handle);

    // The test passes if no panics occur during cleanup
}

#[test]
fn test_drop_guard() {
    use std::sync::atomic::{AtomicBool, Ordering};

    let cleanup_called = Arc::new(AtomicBool::new(false));
    let cleanup_called_clone = cleanup_called.clone();

    {
        let _guard = DropGuard::new(move || {
            cleanup_called_clone.store(true, Ordering::SeqCst);
        });

        // Guard is still alive, cleanup should not have been called
        assert!(!cleanup_called.load(Ordering::SeqCst));
    }

    // Guard is dropped, cleanup should have been called
    assert!(cleanup_called.load(Ordering::SeqCst));
}
