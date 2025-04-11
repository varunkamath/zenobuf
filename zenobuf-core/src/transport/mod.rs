//! Transport layer abstraction for Zenobuf

mod zenoh;

pub use self::zenoh::ZenohTransport;

use crate::error::Result;
use crate::message::Message;
use futures::future::BoxFuture;

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
