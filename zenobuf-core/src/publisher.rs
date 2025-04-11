//! Publisher implementation for Zenobuf

use crate::error::Result;
use crate::message::Message;
use crate::transport;

/// Publisher for Zenobuf
///
/// A Publisher is used to publish messages on a topic.
pub struct Publisher<M: Message> {
    /// Name of the topic
    topic: String,
    /// Inner publisher implementation
    inner: Box<dyn transport::Publisher<M>>,
}

impl<M: Message> Publisher<M> {
    /// Creates a new Publisher
    pub(crate) fn new(topic: String, inner: Box<dyn transport::Publisher<M>>) -> Self {
        Self { topic, inner }
    }

    /// Returns the topic name
    pub fn topic(&self) -> &str {
        &self.topic
    }

    /// Publishes a message
    pub fn publish(&self, message: &M) -> Result<()> {
        self.inner.publish(message)
    }
}
