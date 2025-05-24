//! Transport layer abstraction for Zenobuf

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::error::Result;
use crate::message::Message;
mod zenoh;

pub use self::zenoh::ZenohTransport;

/// A boxed future for async operations
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Transport layer abstraction
///
/// This trait defines the interface that all transport implementations must provide.
/// It allows for pluggable transport layers while maintaining a consistent API.
#[async_trait::async_trait]
pub trait Transport: Send + Sync + 'static {
    /// Create a publisher for the given topic
    async fn create_publisher<M: Message>(&self, topic: &str) -> Result<Arc<crate::publisher::Publisher<M>>>;

    /// Create a subscriber for the given topic with a callback
    async fn create_subscriber<M: Message, F>(
        &self,
        topic: &str,
        callback: F,
    ) -> Result<Arc<crate::subscriber::Subscriber>>
    where
        F: Fn(M) + Send + Sync + 'static;

    /// Create a service for the given service name with a handler
    async fn create_service<Req: Message, Res: Message, F>(
        &self,
        service_name: &str,
        handler: F,
    ) -> Result<Arc<crate::service::Service>>
    where
        F: Fn(Req) -> Result<Res> + Send + Sync + 'static;

    /// Create a client for the given service name
    fn create_client<Req: Message, Res: Message>(
        &self,
        service_name: &str,
    ) -> Result<Arc<crate::client::Client<Req, Res>>>;
}

/// Publisher abstraction
pub trait Publisher<M: Message>: Send + Sync + 'static {
    /// Publishes a message
    fn publish(&self, message: &M) -> Result<()>;
}

/// Subscriber abstraction
pub trait Subscriber: Send + Sync + 'static {
    /// Closes the subscriber
    fn close(&self) -> Result<()>;
}

/// Service abstraction
pub trait Service: Send + Sync + 'static {
    /// Closes the service
    fn close(&self) -> Result<()>;
}

/// Client abstraction
pub trait Client<Req: Message, Res: Message>: Send + Sync + 'static {
    /// Calls the service with the given request
    fn call(&self, request: &Req) -> Result<Res>;

    /// Calls the service with the given request asynchronously
    fn call_async<'a>(&'a self, request: &'a Req) -> BoxFuture<'a, Result<Res>>;
}
