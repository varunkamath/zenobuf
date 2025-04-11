//! Message trait and utilities for working with Protocol Buffer messages

// No imports needed
use prost::Message as ProstMessage;

use crate::error::{Error, Result};

/// Trait for Zenobuf messages
///
/// This trait is implemented for all Protocol Buffer messages that can be used
/// with the Zenobuf framework. It provides methods for serialization and
/// deserialization, as well as type information.
///
/// Users can implement this trait for their own Protocol Buffer messages, or
/// use the `ZenobufMessage` derive macro from the `zenobuf-macros` crate.
pub trait Message: ProstMessage + Default + Clone + Send + Sync + 'static {
    /// Returns the type name of the message
    ///
    /// This is used for type checking and debugging.
    fn type_name() -> &'static str;

    /// Decodes a message from a byte slice
    ///
    /// This is a convenience method that calls `prost::Message::decode`.
    fn decode_from_slice(bytes: &[u8]) -> Result<Self> {
        Self::decode(bytes).map_err(Error::from)
    }
}

/// Helper function to encode a message to a byte vector
pub fn encode_message<M: Message>(message: &M) -> Vec<u8> {
    let mut buf = Vec::with_capacity(message.encoded_len());
    message.encode(&mut buf).expect("Failed to encode message");
    buf
}

/// Helper function to decode a message from a byte slice
pub fn decode_message<M: Message>(bytes: &[u8]) -> Result<M> {
    M::decode_from_slice(bytes)
}

/// Helper function to get the type name of a message
pub fn message_type_name<M: Message>() -> &'static str {
    M::type_name()
}
