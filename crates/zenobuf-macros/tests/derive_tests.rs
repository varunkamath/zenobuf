use prost::Message as ProstMessage;
use zenobuf_core::message::Message;
use zenobuf_macros::ZenobufMessage;

// Define a test message with the derive macro
#[derive(Clone, PartialEq, Debug, Default, ZenobufMessage)]
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

#[test]
fn test_derive_macro() {
    // Check that the Message trait is implemented
    assert_eq!(TestMessage::type_name(), "TestMessage");

    // Create a message
    let message = TestMessage { value: 42 };

    // Encode the message
    let mut buf = Vec::new();
    message.encode(&mut buf).unwrap();

    // Decode the message
    let decoded = TestMessage::decode(buf.as_slice()).unwrap();

    // Check that the decoded message matches the original
    assert_eq!(decoded.value, message.value);
}
