//! Tests for the new builder pattern API

use prost::Message as ProstMessage;
use std::sync::{Arc, Mutex};
use zenobuf_core::message::Message;
use zenobuf_core::transport::ZenohTransport;
use zenobuf_core::{Node, QosPreset, QosProfile};

// Test message type
#[derive(Clone, PartialEq, Debug, Default)]
struct TestMessage {
    value: i32,
    text: String,
}

// Implement ProstMessage for TestMessage
impl ProstMessage for TestMessage {
    fn encode(
        &self,
        buf: &mut impl prost::bytes::BufMut,
    ) -> std::result::Result<(), prost::EncodeError> {
        // Simple encoding for testing
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
    fn encode(
        &self,
        buf: &mut impl prost::bytes::BufMut,
    ) -> std::result::Result<(), prost::EncodeError> {
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
    fn encode(
        &self,
        buf: &mut impl prost::bytes::BufMut,
    ) -> std::result::Result<(), prost::EncodeError> {
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
async fn test_publisher_builder_api() {
    let transport = ZenohTransport::new().await.unwrap();
    let node = Node::with_transport("test_node", transport).unwrap();

    // Test basic builder pattern
    let publisher = node
        .publisher::<TestMessage>("test_topic")
        .build()
        .await
        .unwrap();

    assert_eq!(publisher.topic(), "test_topic");

    // Test builder with QoS preset
    let publisher2 = node
        .publisher::<TestMessage>("test_topic2")
        .with_qos_preset(QosPreset::SensorData)
        .build()
        .await
        .unwrap();

    assert_eq!(publisher2.topic(), "test_topic2");

    // Test builder with custom QoS
    let publisher3 = node
        .publisher::<TestMessage>("test_topic3")
        .reliable()
        .with_depth(50)
        .build()
        .await
        .unwrap();

    assert_eq!(publisher3.topic(), "test_topic3");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_subscriber_builder_api() {
    let transport = ZenohTransport::new().await.unwrap();
    let node = Node::with_transport("test_node", transport).unwrap();

    let received = Arc::new(Mutex::new(None));
    let received_clone = received.clone();

    // Test basic builder pattern
    let _subscriber = node
        .subscriber::<TestMessage>("test_topic")
        .build(move |msg: TestMessage| {
            let mut received = received_clone.lock().unwrap();
            *received = Some(msg);
        })
        .await
        .unwrap();

    // Test builder with QoS preset
    let received2 = Arc::new(Mutex::new(None));
    let received2_clone = received2.clone();

    let _subscriber2 = node
        .subscriber::<TestMessage>("test_topic2")
        .with_qos_preset(QosPreset::HighThroughput)
        .build(move |msg: TestMessage| {
            let mut received = received2_clone.lock().unwrap();
            *received = Some(msg);
        })
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_service_builder_api() {
    let transport = ZenohTransport::new().await.unwrap();
    let node = Node::with_transport("test_node", transport).unwrap();

    // Test basic service builder
    let _service = node
        .service::<AddRequest, AddResponse>("add_service")
        .build(|req: AddRequest| Ok(AddResponse { sum: req.a + req.b }))
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_client_builder_api() {
    let transport = ZenohTransport::new().await.unwrap();
    let node = Node::with_transport("test_node", transport).unwrap();

    // Test basic client builder
    let _client = node
        .client::<AddRequest, AddResponse>("add_service")
        .build()
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_simplified_convenience_methods() {
    let transport = ZenohTransport::new().await.unwrap();
    let node = Node::with_transport("test_node", transport).unwrap();

    // Test simplified publish method
    let publisher = node.publish::<TestMessage>("simple_topic").await.unwrap();
    assert_eq!(publisher.topic(), "simple_topic");

    // Test simplified subscribe method
    let received = Arc::new(Mutex::new(None));
    let received_clone = received.clone();

    let _subscriber = node
        .subscribe::<TestMessage, _>("simple_topic", move |msg: TestMessage| {
            let mut received = received_clone.lock().unwrap();
            *received = Some(msg);
        })
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_qos_preset_conversion() {
    // Test that QosPreset converts correctly to QosProfile
    let default_qos: QosProfile = QosPreset::Default.into();
    assert_eq!(default_qos.depth, 10);

    let sensor_qos: QosProfile = QosPreset::SensorData.into();
    assert_eq!(sensor_qos.depth, 5);

    let high_throughput_qos: QosProfile = QosPreset::HighThroughput.into();
    assert_eq!(high_throughput_qos.depth, 100);

    let low_latency_qos: QosProfile = QosPreset::LowLatency.into();
    assert_eq!(low_latency_qos.depth, 1);
}
