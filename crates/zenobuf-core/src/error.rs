//! Error types for the Zenobuf framework

use thiserror::Error;

/// Result type for Zenobuf operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for Zenobuf operations
#[derive(Error, Debug)]
pub enum Error {
    /// Error from the Zenoh transport layer
    #[error("Transport error in {context}")]
    Transport {
        #[source]
        source: zenoh::Error,
        context: String,
    },

    /// Error during message serialization
    #[error("Message serialization failed for type {type_name}")]
    MessageSerialization {
        #[source]
        source: prost::EncodeError,
        type_name: &'static str,
    },

    /// Error during message deserialization
    #[error("Message deserialization failed for type {type_name}")]
    MessageDeserialization {
        #[source]
        source: prost::DecodeError,
        type_name: &'static str,
    },

    /// Error during serialization or deserialization (legacy)
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Error during message encoding (legacy)
    #[error("Encoding error: {0}")]
    Encoding(#[from] prost::EncodeError),

    /// Error during message decoding (legacy)
    #[error("Decoding error: {0}")]
    Decoding(#[from] prost::DecodeError),

    /// Error when a node with the same name already exists
    #[error("Node '{name}' already exists")]
    NodeAlreadyExists { name: String },

    /// Error when a topic with the same name already exists
    #[error("Topic '{topic}' already exists on node '{node}'")]
    TopicAlreadyExists { topic: String, node: String },

    /// Error when a service with the same name already exists
    #[error("Service '{service}' already exists on node '{node}'")]
    ServiceAlreadyExists { service: String, node: String },

    /// Error when a service call times out
    #[error("Service call to '{service}' timed out after {timeout_ms}ms")]
    ServiceCallTimeout {
        service: String,
        timeout_ms: u64,
    },

    /// Error when a service call fails
    #[error("Service call to '{service}' failed: {reason}")]
    ServiceCallFailed { service: String, reason: String },

    /// Error when a parameter operation fails
    #[error("Parameter '{name}' error: {reason}")]
    Parameter { name: String, reason: String },

    /// Error when a node operation fails
    #[error("Node '{node}' error: {reason}")]
    Node { node: String, reason: String },

    /// Error when a publisher operation fails
    #[error("Publisher for topic '{topic}' error: {reason}")]
    Publisher { topic: String, reason: String },

    /// Error when a subscriber operation fails
    #[error("Subscriber for topic '{topic}' error: {reason}")]
    Subscriber { topic: String, reason: String },

    /// Error when a service operation fails
    #[error("Service '{service}' error: {reason}")]
    Service { service: String, reason: String },

    /// Error when a client operation fails
    #[error("Client for service '{service}' error: {reason}")]
    Client { service: String, reason: String },

    /// Error when an operation is not supported
    #[error("Operation '{operation}' not supported: {reason}")]
    NotSupported { operation: String, reason: String },

    /// Error when an operation is not implemented
    #[error("Operation '{operation}' not implemented: {reason}")]
    NotImplemented { operation: String, reason: String },

    /// Configuration error
    #[error("Configuration error: {reason}")]
    Configuration { reason: String },

    /// Network error
    #[error("Network error: {reason}")]
    Network { reason: String },

    /// Other errors
    #[error("Error: {reason}")]
    Other { reason: String },

    // Legacy string-based errors for backward compatibility
    #[error("Node already exists: {0}")]
    NodeAlreadyExistsLegacy(String),

    #[error("Topic already exists: {0}")]
    TopicAlreadyExistsLegacy(String),

    #[error("Service already exists: {0}")]
    ServiceAlreadyExistsLegacy(String),

    #[error("Service call timed out: {0}")]
    ServiceCallTimeoutLegacy(String),

    #[error("Service call failed: {0}")]
    ServiceCallFailedLegacy(String),

    #[error("Parameter error: {0}")]
    ParameterLegacy(String),

    #[error("Node error: {0}")]
    NodeLegacy(String),

    #[error("Publisher error: {0}")]
    PublisherLegacy(String),

    #[error("Subscriber error: {0}")]
    SubscriberLegacy(String),

    #[error("Service error: {0}")]
    ServiceLegacy(String),

    #[error("Client error: {0}")]
    ClientLegacy(String),

    #[error("Operation not supported: {0}")]
    NotSupportedLegacy(String),

    #[error("Operation not implemented: {0}")]
    NotImplementedLegacy(String),

    #[error("Other error: {0}")]
    OtherLegacy(String),
}

// Legacy From implementations for backward compatibility
impl From<zenoh::Error> for Error {
    fn from(err: zenoh::Error) -> Self {
        Error::Transport {
            source: err,
            context: "unknown".to_string(),
        }
    }
}

// Error context helpers
pub trait ErrorContext<T> {
    /// Add context to an error
    fn with_context(self, context: &str) -> Result<T>;
    /// Add context with format arguments
    fn with_context_f<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;
}

impl<T> ErrorContext<T> for Result<T> {
    fn with_context(self, context: &str) -> Result<T> {
        self.map_err(|err| match err {
            Error::Transport { source, .. } => Error::Transport {
                source,
                context: context.to_string(),
            },
            other => other,
        })
    }

    fn with_context_f<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|err| match err {
            Error::Transport { source, .. } => Error::Transport {
                source,
                context: f(),
            },
            other => other,
        })
    }
}

// Helper functions for creating structured errors
impl Error {
    /// Create a transport error with context
    pub fn transport(source: zenoh::Error, context: impl Into<String>) -> Self {
        Error::Transport {
            source,
            context: context.into(),
        }
    }

    /// Create a message serialization error
    pub fn message_serialization(
        source: prost::EncodeError,
        type_name: &'static str,
    ) -> Self {
        Error::MessageSerialization { source, type_name }
    }

    /// Create a message deserialization error
    pub fn message_deserialization(
        source: prost::DecodeError,
        type_name: &'static str,
    ) -> Self {
        Error::MessageDeserialization { source, type_name }
    }

    /// Create a node already exists error
    pub fn node_already_exists(name: impl Into<String>) -> Self {
        Error::NodeAlreadyExists { name: name.into() }
    }

    /// Create a topic already exists error
    pub fn topic_already_exists(
        topic: impl Into<String>,
        node: impl Into<String>,
    ) -> Self {
        Error::TopicAlreadyExists {
            topic: topic.into(),
            node: node.into(),
        }
    }

    /// Create a service already exists error
    pub fn service_already_exists(
        service: impl Into<String>,
        node: impl Into<String>,
    ) -> Self {
        Error::ServiceAlreadyExists {
            service: service.into(),
            node: node.into(),
        }
    }

    /// Create a service call timeout error
    pub fn service_call_timeout(
        service: impl Into<String>,
        timeout_ms: u64,
    ) -> Self {
        Error::ServiceCallTimeout {
            service: service.into(),
            timeout_ms,
        }
    }

    /// Create a service call failed error
    pub fn service_call_failed(
        service: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Error::ServiceCallFailed {
            service: service.into(),
            reason: reason.into(),
        }
    }

    /// Create a parameter error
    pub fn parameter(name: impl Into<String>, reason: impl Into<String>) -> Self {
        Error::Parameter {
            name: name.into(),
            reason: reason.into(),
        }
    }

    /// Create a node error
    pub fn node(node: impl Into<String>, reason: impl Into<String>) -> Self {
        Error::Node {
            node: node.into(),
            reason: reason.into(),
        }
    }

    /// Create a publisher error
    pub fn publisher(topic: impl Into<String>, reason: impl Into<String>) -> Self {
        Error::Publisher {
            topic: topic.into(),
            reason: reason.into(),
        }
    }

    /// Create a subscriber error
    pub fn subscriber(topic: impl Into<String>, reason: impl Into<String>) -> Self {
        Error::Subscriber {
            topic: topic.into(),
            reason: reason.into(),
        }
    }

    /// Create a service error
    pub fn service(service: impl Into<String>, reason: impl Into<String>) -> Self {
        Error::Service {
            service: service.into(),
            reason: reason.into(),
        }
    }

    /// Create a client error
    pub fn client(service: impl Into<String>, reason: impl Into<String>) -> Self {
        Error::Client {
            service: service.into(),
            reason: reason.into(),
        }
    }

    /// Create a configuration error
    pub fn configuration(reason: impl Into<String>) -> Self {
        Error::Configuration {
            reason: reason.into(),
        }
    }

    /// Create a network error
    pub fn network(reason: impl Into<String>) -> Self {
        Error::Network {
            reason: reason.into(),
        }
    }

    /// Create an other error
    pub fn other(reason: impl Into<String>) -> Self {
        Error::Other {
            reason: reason.into(),
        }
    }
}
