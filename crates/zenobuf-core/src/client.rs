//! Client implementation for Zenobuf

use futures::future::BoxFuture;

use crate::error::Result;
use crate::message::Message;
use crate::transport;

/// Client for Zenobuf
///
/// A Client is used to send requests to a service and receive responses.
pub struct Client<Req: Message, Res: Message> {
    /// Name of the service
    name: String,
    /// Inner client implementation
    inner: Box<dyn transport::Client<Req, Res>>,
}

impl<Req: Message, Res: Message> Client<Req, Res> {
    /// Creates a new Client
    pub(crate) fn new(name: String, inner: Box<dyn transport::Client<Req, Res>>) -> Self {
        Self { name, inner }
    }

    /// Returns the service name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Calls the service with the given request
    pub fn call(&self, request: &Req) -> Result<Res> {
        self.inner.call(request)
    }

    /// Calls the service with the given request asynchronously
    pub fn call_async<'a>(&'a self, request: &'a Req) -> BoxFuture<'a, Result<Res>> {
        self.inner.call_async(request)
    }
}
