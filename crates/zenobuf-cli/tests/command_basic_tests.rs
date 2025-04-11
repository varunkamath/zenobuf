use std::sync::Arc;

use zenobuf_core::node::Node;
use zenobuf_core::transport::ZenohTransport;

// Import the test messages
mod test_messages {
    use prost::Message as ProstMessage;
    use zenobuf_core::message::Message;

    // Define a test message
    #[derive(Clone, PartialEq, Debug, Default)]
    pub struct TestMessage {
        pub value: i32,
        pub text: String,
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
    pub struct AddRequest {
        pub a: i32,
        pub b: i32,
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
    pub struct AddResponse {
        pub sum: i32,
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
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_basic_commands() {
    // Create a transport
    let transport = ZenohTransport::new().await.unwrap();

    // Create a node
    let node = Arc::new(Node::with_transport("test_node", transport).unwrap());

    // Set a parameter
    node.set_parameter("test_param", 42).unwrap();

    // Get the parameter
    let value = node.get_parameter::<i32>("test_param").unwrap();
    assert_eq!(value, 42);
}
