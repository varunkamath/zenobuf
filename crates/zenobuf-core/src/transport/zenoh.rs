//! Zenoh transport implementation for Zenobuf

use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use zenoh::qos::{CongestionControl, Priority};
use zenoh::{self, key_expr::KeyExpr};

use crate::error::{Error, Result};
use crate::executor::CallbackExecutor;
use crate::message::{decode_message, encode_message, Message};
use crate::qos::{QosProfile, Reliability};

use super::{BoxFuture, Client, Publisher, Service, Subscriber};

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

    /// Returns a reference to the Zenoh session
    pub(crate) fn session(&self) -> &Arc<zenoh::Session> {
        &self.session
    }

    /// Maps Zenobuf reliability to Zenoh CongestionControl
    fn map_reliability(qos: &QosProfile) -> CongestionControl {
        match qos.reliability {
            Reliability::Reliable => CongestionControl::Block,
            Reliability::BestEffort => CongestionControl::Drop,
        }
    }

    /// Creates a publisher for the given topic with QoS settings
    pub async fn create_publisher<M: Message>(
        &self,
        topic: &str,
        qos: &QosProfile,
    ) -> Result<ZenohPublisher<M>> {
        let prefixed_topic = format!("{}{topic}", Self::TOPIC_PREFIX);
        ZenohPublisher::new(
            self.session.clone(),
            prefixed_topic,
            Self::map_reliability(qos),
            Priority::Data,
        )
        .await
    }

    /// Creates a subscriber for the given topic
    pub async fn create_subscriber<M: Message, F>(
        &self,
        topic: &str,
        callback: F,
        executor: Option<Arc<CallbackExecutor>>,
    ) -> Result<ZenohSubscriber>
    where
        F: Fn(M) + Send + Sync + 'static,
    {
        let prefixed_topic = format!("{}{topic}", Self::TOPIC_PREFIX);
        ZenohSubscriber::new(self.session.clone(), &prefixed_topic, callback, executor).await
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
        let prefixed_service_name = format!("{}{service_name}", Self::SERVICE_PREFIX);
        ZenohService::new(self.session.clone(), &prefixed_service_name, handler).await
    }

    /// Creates a client for the given service name
    pub fn create_client<Req: Message, Res: Message>(
        &self,
        service_name: &str,
    ) -> Result<ZenohClient<Req, Res>> {
        let prefixed_service_name = format!("{}{service_name}", Self::SERVICE_PREFIX);
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
    /// Creates a new Zenoh publisher with QoS settings
    async fn new(
        session: Arc<zenoh::Session>,
        topic: String,
        congestion_control: CongestionControl,
        priority: Priority,
    ) -> Result<Self> {
        let key_expr = KeyExpr::try_from(topic.clone())
            .map_err(|e| Error::publisher(&topic, e.to_string()))?;
        let publisher = session
            .declare_publisher(key_expr)
            .congestion_control(congestion_control)
            .priority(priority)
            .await
            .map_err(Error::from)?;

        tracing::debug!(
            "Publisher created: topic={}, congestion={:?}, priority={:?}",
            topic,
            congestion_control,
            priority
        );

        Ok(Self {
            publisher,
            _phantom: PhantomData,
        })
    }
}

impl<M: Message> Publisher<M> for ZenohPublisher<M> {
    fn publish(&self, message: &M) -> Result<()> {
        let bytes = encode_message(message)?;
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.publisher.put(bytes).await.map_err(Error::from)
            })
        })
    }
}

/// Zenoh subscriber implementation
pub struct ZenohSubscriber {
    _subscriber: zenoh::pubsub::Subscriber<()>,
}

impl ZenohSubscriber {
    /// Creates a new Zenoh subscriber
    ///
    /// If an executor is provided, callbacks will be queued to it for later processing
    /// by the node's spin methods. Otherwise, callbacks are executed directly in the
    /// Zenoh callback thread.
    async fn new<M: Message, F>(
        session: Arc<zenoh::Session>,
        topic: &str,
        callback: F,
        executor: Option<Arc<CallbackExecutor>>,
    ) -> Result<Self>
    where
        F: Fn(M) + Send + Sync + 'static,
    {
        let key_expr =
            KeyExpr::try_from(topic).map_err(|e| Error::subscriber(topic, e.to_string()))?;

        let callback = Arc::new(callback);

        let subscriber = session
            .declare_subscriber(key_expr)
            .callback(move |sample| {
                let bytes = sample.payload().to_bytes();
                match decode_message::<M>(bytes.as_ref()) {
                    Ok(message) => {
                        if let Some(ref exec) = executor {
                            let cb = callback.clone();
                            exec.enqueue(Box::new(move || cb(message)));
                        } else {
                            callback(message);
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to decode subscriber message: {}", e);
                    }
                }
            })
            .await
            .map_err(Error::from)?;

        Ok(Self {
            _subscriber: subscriber,
        })
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
    _task: tokio::task::JoinHandle<()>,
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

        let task = tokio::spawn(async move {
            while let Ok(query) = queryable_clone.recv_async().await {
                tracing::info!("Received query on: {}", query.key_expr());

                let Some(payload) = query.payload() else {
                    tracing::error!("Query has no payload");
                    let _ = query
                        .reply_err("Query has no payload".as_bytes().to_vec())
                        .await;
                    continue;
                };

                let request = match decode_message::<Req>(payload.to_bytes().as_ref()) {
                    Ok(req) => req,
                    Err(_) => {
                        tracing::error!("Failed to decode request");
                        let _ = query
                            .reply_err("Failed to decode request".as_bytes().to_vec())
                            .await;
                        continue;
                    }
                };

                tracing::info!("Decoded request successfully");
                let response = match handler(request) {
                    Ok(res) => res,
                    Err(e) => {
                        tracing::error!("Service handler error: {}", e);
                        let _ = query
                            .reply_err(format!("Service error: {e}").as_bytes().to_vec())
                            .await;
                        continue;
                    }
                };

                tracing::info!("Handler returned response");
                let bytes = match encode_message(&response) {
                    Ok(b) => b,
                    Err(e) => {
                        tracing::error!("Failed to encode response: {}", e);
                        let _ = query
                            .reply_err(format!("Encode error: {e}").as_bytes().to_vec())
                            .await;
                        continue;
                    }
                };

                match query.reply(query.key_expr(), bytes).await {
                    Ok(_) => tracing::info!("Reply sent successfully"),
                    Err(e) => tracing::error!("Failed to send reply: {}", e),
                }
            }
        });

        Ok(Self {
            _queryable: queryable,
            _task: task,
        })
    }
}

impl Drop for ZenohService {
    fn drop(&mut self) {
        self._task.abort();
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
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.call_async(request))
        })
    }

    fn call_async<'a>(&'a self, request: &'a Req) -> BoxFuture<'a, Result<Res>> {
        let service_name = self.service_name.clone();
        let session = self.session.clone();

        Box::pin(async move {
            let key_expr = KeyExpr::try_from(service_name.clone())
                .map_err(|e| Error::client(&service_name, e.to_string()))?;

            let bytes = encode_message(request)?;
            tracing::info!("Sending request to: {}", key_expr);

            // Implement retry mechanism with exponential backoff
            let max_retries = 3;
            let mut retry_count = 0;
            let mut last_error = None;
            let base_delay = Duration::from_millis(100);

            while retry_count < max_retries {
                // Make a request with a timeout
                match session
                    .get(key_expr.clone())
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

            Err(last_error.unwrap_or_else(|| {
                Error::service_call_failed(&service_name, "Service call failed after retries")
            }))
        })
    }
}
