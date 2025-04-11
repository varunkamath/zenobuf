use prost::Message as ProstMessage;
use zenobuf_core::message::{decode_message, encode_message, Message};

// Define a simple test message
#[derive(Clone, PartialEq, Debug, Default)]
struct TestMessage {
    value: i32,
}

// Implement ProstMessage for TestMessage
impl ProstMessage for TestMessage {
    fn encode(&self, buf: &mut impl prost::bytes::BufMut) -> Result<(), prost::EncodeError> {
        // Simple encoding for testing
        buf.put_slice(&self.value.to_le_bytes());
        Ok(())
    }

    fn decode(buf: impl prost::bytes::Buf) -> Result<Self, prost::DecodeError> {
        let mut buf = buf;
        if buf.remaining() < 4 {
            return Err(prost::DecodeError::new("Buffer too short"));
        }

        let mut bytes = [0u8; 4];
        buf.copy_to_slice(&mut bytes);
        let value = i32::from_le_bytes(bytes);

        Ok(TestMessage { value })
    }

    fn encoded_len(&self) -> usize {
        4 // 4 bytes for value
    }

    fn clear(&mut self) {
        self.value = 0;
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

#[test]
fn test_message_encode_decode() {
    let message = TestMessage { value: 42 };

    // Encode the message
    let bytes = encode_message(&message);

    // Decode the message
    let decoded = decode_message::<TestMessage>(&bytes).unwrap();

    // Check that the decoded message matches the original
    assert_eq!(decoded.value, message.value);
}

#[test]
fn test_message_type_name() {
    assert_eq!(TestMessage::type_name(), "TestMessage");
}

#[test]
fn test_message_decode_error() {
    // Create an invalid byte array
    let bytes = vec![1, 2, 3]; // Too short

    // Try to decode the message
    let result = decode_message::<TestMessage>(&bytes);

    // Check that decoding failed
    assert!(result.is_err());
}

#[test]
fn test_message_default() {
    let message = TestMessage::default();
    assert_eq!(message.value, 0);
}

#[test]
fn test_message_clear() {
    let mut message = TestMessage { value: 42 };

    message.clear();

    assert_eq!(message.value, 0);
}

#[test]
fn test_message_encoded_len() {
    let message = TestMessage { value: 42 };

    assert_eq!(message.encoded_len(), 4); // 4 bytes for value
}
