//! Subscriber implementation for Zenobuf

use crate::error::Result;
use crate::transport;

/// Subscriber for Zenobuf
///
/// A Subscriber is used to receive messages on a topic.
pub struct Subscriber {
    /// Name of the topic
    topic: String,
    /// Inner subscriber implementation
    inner: Box<dyn transport::Subscriber>,
}

impl Subscriber {
    /// Creates a new Subscriber
    pub(crate) fn new(topic: String, inner: Box<dyn transport::Subscriber>) -> Self {
        Self { topic, inner }
    }

    /// Returns the topic name
    pub fn topic(&self) -> &str {
        &self.topic
    }

    /// Closes the subscriber
    pub fn close(&self) -> Result<()> {
        self.inner.close()
    }
}
