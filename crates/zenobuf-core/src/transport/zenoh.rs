//! Zenoh transport implementation for Zenobuf

use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use futures::future::BoxFuture;
use zenoh::{self, key_expr::KeyExpr};

use crate::error::{Error, Result};
use crate::message::{decode_message, encode_message, Message};

use super::{Client, Publisher, Service, Subscriber, Transport};
use async_trait::async_trait;

/// Zenoh transport implementation
pub struct ZenohTransport {
    session: Arc<zenoh::Session>,
}

impl ZenohTransport {
    /// Creates a new Zenoh transport
    pub async fn new() -> Result<Self> {
        let config = zenoh::config::Config::default();
        let session = zenoh::open(config).await.map_err(Error::from)?;
        Ok(Self {
            session: Arc::new(session),
        })
    }

    /// Prefixes for Zenoh key expressions
    pub const TOPIC_PREFIX: &str = "zenobuf/topic/";
    pub const SERVICE_PREFIX: &str = "zenobuf/service/";

    /// Creates a new Zenoh transport with the given configuration
    pub async fn with_config(config: zenoh::config::Config) -> Result<Self> {
        let session = zenoh::open(config).await.map_err(Error::from)?;
        Ok(Self {
            session: Arc::new(session),
        })
    }

    /// Creates a publisher for the given topic
    pub async fn create_publisher<M: Message>(&self, topic: &str) -> Result<ZenohPublisher<M>> {
        let prefixed_topic = format!(
            "{prefix}{topic}",
            prefix = Self::TOPIC_PREFIX,
            topic = topic
        );
        ZenohPublisher::new(self.session.clone(), prefixed_topic).await
    }

    /// Creates a subscriber for the given topic
    pub async fn create_subscriber<M: Message, F>(
        &self,
        topic: &str,
        callback: F,
    ) -> Result<ZenohSubscriber>
    where
        F: Fn(M) + Send + Sync + 'static,
    {
        let prefixed_topic = format!(
            "{prefix}{topic}",
            prefix = Self::TOPIC_PREFIX,
            topic = topic
        );
        ZenohSubscriber::new(self.session.clone(), &prefixed_topic, callback).await
    }

    /// Creates a service for the given name
    pub async fn create_service<Req: Message, Res: Message, F>(
        &self,
        service_name: &str,
        handler: F,
    ) -> Result<ZenohService>
    where
        F: Fn(Req) -> Result<Res> + Send + Sync + 'static,
    {
        let prefixed_service_name = format!(
            "{prefix}{service_name}",
            prefix = Self::SERVICE_PREFIX,
            service_name = service_name
        );
        ZenohService::new(self.session.clone(), &prefixed_service_name, handler).await
    }

    /// Creates a client for the given service name
    pub fn create_client<Req: Message, Res: Message>(
        &self,
        service_name: &str,
    ) -> Result<ZenohClient<Req, Res>> {
        let prefixed_service_name = format!(
            "{prefix}{service_name}",
            prefix = Self::SERVICE_PREFIX,
            service_name = service_name
        );
        Ok(ZenohClient::new(
            self.session.clone(),
            &prefixed_service_name,
        ))
    }
}

/// Zenoh publisher implementation
pub struct ZenohPublisher<M: Message> {
    publisher: zenoh::pubsub::Publisher<'static>,
    _phantom: PhantomData<M>,
}

impl<M: Message> ZenohPublisher<M> {
    /// Creates a new Zenoh publisher
    async fn new(session: Arc<zenoh::Session>, topic: String) -> Result<Self> {
        let topic_clone = topic.clone();
        let key_expr = KeyExpr::try_from(topic.clone())
            .map_err(|e| Error::publisher(topic_clone, e.to_string()))?;
        let publisher = session
            .declare_publisher(key_expr)
            .await
            .map_err(Error::from)?;

        Ok(Self {
            publisher,
            _phantom: PhantomData,
        })
    }
}

impl<M: Message> Publisher<M> for ZenohPublisher<M> {
    fn publish(&self, message: &M) -> Result<()> {
        let bytes = encode_message(message);
        // Use futures::executor::block_on instead of creating a new Tokio runtime
        // This works in both async and sync contexts
        futures::executor::block_on(async {
            self.publisher.put(bytes).await.map_err(Error::from)
        })?;
        Ok(())
    }

    // TODO: Consider adding an explicit async version of this method in the future
    // for better ergonomics in async contexts
}

/// Zenoh subscriber implementation
pub struct ZenohSubscriber {
    _subscriber: zenoh::pubsub::Subscriber<()>,
}

impl ZenohSubscriber {
    /// Creates a new Zenoh subscriber
    async fn new<M: Message, F>(
        session: Arc<zenoh::Session>,
        topic: &str,
        callback: F,
    ) -> Result<Self>
    where
        F: Fn(M) + Send + Sync + 'static,
    {
        let key_expr =
            KeyExpr::try_from(topic).map_err(|e| Error::subscriber(topic, e.to_string()))?;
        let subscriber = session
            .declare_subscriber(key_expr)
            .callback(move |sample| {
                let bytes = sample.payload().to_bytes();
                if let Ok(message) = decode_message::<M>(bytes.as_ref()) {
                    callback(message);
                }
            })
            .await
            .map_err(Error::from)?;

        // We need to modify our struct definition to match what Zenoh returns
        // For now, let's just store the subscriber directly
        let result = Self {
            _subscriber: subscriber,
        };

        Ok(result)
    }
}

impl Subscriber for ZenohSubscriber {
    fn close(&self) -> Result<()> {
        // The subscriber will be closed when it's dropped
        Ok(())
    }
}

/// Zenoh service implementation
pub struct ZenohService {
    _queryable: zenoh::query::Queryable<zenoh::handlers::FifoChannelHandler<zenoh::query::Query>>,
}

impl ZenohService {
    /// Creates a new Zenoh service
    async fn new<Req: Message, Res: Message, F>(
        session: Arc<zenoh::Session>,
        service_name: &str,
        handler: F,
    ) -> Result<Self>
    where
        F: Fn(Req) -> Result<Res> + Send + Sync + 'static,
    {
        let key_expr = KeyExpr::try_from(service_name)
            .map_err(|e| Error::service(service_name, e.to_string()))?;
        tracing::info!("Declaring service: {}", service_name);
        let queryable = session
            .declare_queryable(key_expr)
            .await
            .map_err(Error::from)?;

        // Clone the queryable for the task
        let queryable_clone = queryable.clone();

        // Spawn a task to handle queries
        tokio::spawn(async move {
            while let Ok(query) = queryable_clone.recv_async().await {
                tracing::info!("Received query on: {}", query.key_expr());
                if let Some(payload) = query.payload() {
                    tracing::info!("Query has payload");
                    if let Ok(request) = decode_message::<Req>(payload.to_bytes().as_ref()) {
                        tracing::info!("Decoded request successfully");
                        match handler(request) {
                            Ok(response) => {
                                tracing::info!("Handler returned response");
                                let bytes = encode_message(&response);
                                // Send the reply immediately
                                match query.reply(query.key_expr(), bytes).await {
                                    Ok(_) => tracing::info!("Reply sent successfully"),
                                    Err(e) => tracing::error!("Failed to send reply: {}", e),
                                }
                            }
                            Err(e) => {
                                tracing::error!("Service handler error: {}", e);
                                // Try to send an error reply
                                let _ = query
                                    .reply_err(format!("Service error: {e}").as_bytes().to_vec())
                                    .await;
                            }
                        }
                    } else {
                        tracing::error!("Failed to decode request");
                        // Send an error reply for decoding failure
                        let _ = query
                            .reply_err("Failed to decode request".as_bytes().to_vec())
                            .await;
                    }
                } else {
                    tracing::error!("Query has no payload");
                    // Send an error reply for missing payload
                    let _ = query
                        .reply_err("Query has no payload".as_bytes().to_vec())
                        .await;
                }
            }
        });

        Ok(Self {
            _queryable: queryable,
        })
    }
}

impl Service for ZenohService {
    fn close(&self) -> Result<()> {
        // The queryable will be closed when it's dropped
        Ok(())
    }
}

/// Zenoh client implementation
pub struct ZenohClient<Req: Message, Res: Message> {
    session: Arc<zenoh::Session>,
    service_name: String,
    _phantom: PhantomData<(Req, Res)>,
}

impl<Req: Message, Res: Message> ZenohClient<Req, Res> {
    /// Creates a new Zenoh client
    fn new(session: Arc<zenoh::Session>, service_name: &str) -> Self {
        Self {
            session,
            service_name: service_name.to_string(),
            _phantom: PhantomData,
        }
    }
}

impl<Req: Message, Res: Message> Client<Req, Res> for ZenohClient<Req, Res> {
    fn call(&self, request: &Req) -> Result<Res> {
        // Use futures::executor::block_on instead of creating a new Tokio runtime
        // This works in both async and sync contexts
        futures::executor::block_on(async {
            tracing::info!("Calling service: {}", self.service_name);
            let service_name_clone = self.service_name.clone();
            let key_expr = KeyExpr::try_from(self.service_name.clone())
                .map_err(|e| Error::client(service_name_clone, e.to_string()))?;

            let bytes = encode_message(request);
            let selector = key_expr.clone();
            tracing::info!("Sending request to: {}", selector);

            // Implement retry mechanism with exponential backoff
            let max_retries = 3;
            let mut retry_count = 0;
            let mut last_error = None;
            let base_delay = Duration::from_millis(100);

            while retry_count < max_retries {
                // Make a request with a timeout
                match self
                    .session
                    .get(selector.clone())
                    .payload(bytes.clone())
                    .timeout(Duration::from_secs(10)) // Use a reasonable timeout
                    .await
                {
                    Ok(reply) => {
                        tracing::info!("Got reply, waiting for data");

                        // Keep the reply object alive until we've received the response
                        match reply.recv_async().await {
                            Ok(sample) => match sample.result() {
                                Ok(sample) => {
                                    tracing::info!("Sample is OK");
                                    let payload_data = sample.payload();
                                    tracing::info!("Got payload data");
                                    match decode_message::<Res>(payload_data.to_bytes().as_ref()) {
                                        Ok(response) => {
                                            tracing::info!("Decoded response successfully");
                                            return Ok(response);
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to decode response: {}", e);
                                            last_error = Some(e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Sample error: {}", e);
                                    last_error = Some(Error::service_call_failed(
                                        self.service_name.clone(),
                                        format!("Error in response: {e}"),
                                    ));
                                }
                            },
                            Err(e) => {
                                tracing::error!("Receive error: {}", e);
                                last_error = Some(Error::service_call_failed(
                                    self.service_name.clone(),
                                    format!("No response: {e}"),
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Error getting reply: {}", e);
                        last_error = Some(Error::from(e));
                    }
                }

                // Increment retry count and wait before retrying
                retry_count += 1;
                if retry_count < max_retries {
                    tracing::info!(
                        "Retrying service call (attempt {}/{})",
                        retry_count + 1,
                        max_retries
                    );
                    // Use exponential backoff
                    let backoff = base_delay * 2u32.pow(retry_count as u32);
                    tracing::info!("Waiting for {:?} before retry", backoff);
                    tokio::time::sleep(backoff).await;
                }
            }

            // If we've exhausted all retries, return the last error
            match last_error {
                Some(e) => Err(e),
                None => Err(Error::service_call_failed(
                    self.service_name.clone(),
                    "Service call failed after retries",
                )),
            }
        })
    }

    fn call_async<'a>(&'a self, request: &'a Req) -> BoxFuture<'a, Result<Res>> {
        let service_name = self.service_name.clone();
        let session = self.session.clone();

        Box::pin(async move {
            let service_name_clone = service_name.clone();
            let key_expr = KeyExpr::try_from(service_name.clone())
                .map_err(|e| Error::client(service_name_clone, e.to_string()))?;

            let bytes = encode_message(request);
            let selector = key_expr.clone();
            tracing::info!("Sending request to: {}", selector);

            // Implement retry mechanism with exponential backoff
            let max_retries = 3;
            let mut retry_count = 0;
            let mut last_error = None;
            let base_delay = Duration::from_millis(100);

            while retry_count < max_retries {
                // Make a request with a timeout
                match session
                    .get(selector.clone())
                    .payload(bytes.clone())
                    .timeout(Duration::from_secs(10)) // Use a reasonable timeout
                    .await
                {
                    Ok(reply) => {
                        tracing::info!("Got reply, waiting for data");

                        // Keep the reply object alive until we've received the response
                        match reply.recv_async().await {
                            Ok(sample) => match sample.result() {
                                Ok(sample) => {
                                    tracing::info!("Sample is OK");
                                    let payload_data = sample.payload();
                                    tracing::info!("Got payload data");
                                    match decode_message::<Res>(payload_data.to_bytes().as_ref()) {
                                        Ok(response) => {
                                            tracing::info!("Decoded response successfully");
                                            return Ok(response);
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to decode response: {}", e);
                                            last_error = Some(e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Sample error: {}", e);
                                    last_error = Some(Error::service_call_failed(
                                        service_name.clone(),
                                        format!("Error in response: {e}"),
                                    ));
                                }
                            },
                            Err(e) => {
                                tracing::error!("Receive error: {}", e);
                                last_error = Some(Error::service_call_failed(
                                    service_name.clone(),
                                    format!("No response: {e}"),
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Error getting reply: {}", e);
                        last_error = Some(Error::from(e));
                    }
                }

                // Increment retry count and wait before retrying
                retry_count += 1;
                if retry_count < max_retries {
                    tracing::info!(
                        "Retrying service call (attempt {}/{})",
                        retry_count + 1,
                        max_retries
                    );
                    // Use exponential backoff
                    let backoff = base_delay * 2u32.pow(retry_count as u32);
                    tracing::info!("Waiting for {:?} before retry", backoff);
                    tokio::time::sleep(backoff).await;
                }
            }

            // If we've exhausted all retries, return the last error
            match last_error {
                Some(e) => Err(e),
                None => Err(Error::service_call_failed(
                    service_name.clone(),
                    "Service call failed after retries",
                )),
            }
        })
    }
}

// Implement the Transport trait for ZenohTransport
#[async_trait]
impl Transport for ZenohTransport {
    async fn create_publisher<M: Message>(
        &self,
        topic: &str,
    ) -> Result<Arc<crate::publisher::Publisher<M>>> {
        let zenoh_publisher = ZenohPublisher::new(self.session.clone(), topic.to_string()).await?;
        Ok(Arc::new(crate::publisher::Publisher::new(
            topic.to_string(),
            Box::new(zenoh_publisher),
        )))
    }

    async fn create_subscriber<M: Message, F>(
        &self,
        topic: &str,
        callback: F,
    ) -> Result<Arc<crate::subscriber::Subscriber>>
    where
        F: Fn(M) + Send + Sync + 'static,
    {
        let zenoh_subscriber = ZenohSubscriber::new(self.session.clone(), topic, callback).await?;
        Ok(Arc::new(crate::subscriber::Subscriber::new(
            topic.to_string(),
            Box::new(zenoh_subscriber),
        )))
    }

    async fn create_service<Req: Message, Res: Message, F>(
        &self,
        service_name: &str,
        handler: F,
    ) -> Result<Arc<crate::service::Service>>
    where
        F: Fn(Req) -> Result<Res> + Send + Sync + 'static,
    {
        let zenoh_service = ZenohService::new(self.session.clone(), service_name, handler).await?;
        Ok(Arc::new(crate::service::Service::new(
            service_name.to_string(),
            Box::new(zenoh_service),
        )))
    }

    fn create_client<Req: Message, Res: Message>(
        &self,
        service_name: &str,
    ) -> Result<Arc<crate::client::Client<Req, Res>>> {
        let zenoh_client = ZenohClient::new(self.session.clone(), service_name);
        Ok(Arc::new(crate::client::Client::new(
            service_name.to_string(),
            Box::new(zenoh_client),
        )))
    }
}
