//! Service implementation for Zenobuf

use crate::error::Result;
use crate::transport;

/// Service for Zenobuf
///
/// A Service is used to handle requests and send responses.
pub struct Service {
    /// Name of the service
    name: String,
    /// Inner service implementation
    inner: Box<dyn transport::Service>,
}

impl Service {
    /// Creates a new Service
    pub(crate) fn new(name: String, inner: Box<dyn transport::Service>) -> Self {
        Self { name, inner }
    }

    /// Returns the service name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Closes the service
    pub fn close(&self) -> Result<()> {
        self.inner.close()
    }
}
