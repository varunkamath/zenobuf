//! Error types for the Zenobuf framework

use thiserror::Error;

/// Result type for Zenobuf operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for Zenobuf operations
#[derive(Error, Debug)]
pub enum Error {
    /// Error from the Zenoh transport layer
    #[error("Zenoh error: {0}")]
    Zenoh(#[from] zenoh::Error),

    /// Error during serialization or deserialization
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Error during message encoding
    #[error("Encoding error: {0}")]
    Encoding(#[from] prost::EncodeError),

    /// Error during message decoding
    #[error("Decoding error: {0}")]
    Decoding(#[from] prost::DecodeError),

    /// Error when a node with the same name already exists
    #[error("Node already exists: {0}")]
    NodeAlreadyExists(String),

    /// Error when a topic with the same name already exists
    #[error("Topic already exists: {0}")]
    TopicAlreadyExists(String),

    /// Error when a service with the same name already exists
    #[error("Service already exists: {0}")]
    ServiceAlreadyExists(String),

    /// Error when a service call times out
    #[error("Service call timed out: {0}")]
    ServiceCallTimeout(String),

    /// Error when a service call fails
    #[error("Service call failed: {0}")]
    ServiceCallFailed(String),

    /// Error when a parameter operation fails
    #[error("Parameter error: {0}")]
    Parameter(String),

    /// Error when a node operation fails
    #[error("Node error: {0}")]
    Node(String),

    /// Error when a publisher operation fails
    #[error("Publisher error: {0}")]
    Publisher(String),

    /// Error when a subscriber operation fails
    #[error("Subscriber error: {0}")]
    Subscriber(String),

    /// Error when a service operation fails
    #[error("Service error: {0}")]
    Service(String),

    /// Error when a client operation fails
    #[error("Client error: {0}")]
    Client(String),

    /// Error when an operation is not supported
    #[error("Operation not supported: {0}")]
    NotSupported(String),

    /// Error when an operation is not implemented
    #[error("Operation not implemented: {0}")]
    NotImplemented(String),

    /// Other errors
    #[error("Other error: {0}")]
    Other(String),
}
