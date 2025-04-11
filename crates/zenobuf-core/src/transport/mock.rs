//! Mock transport implementation for testing

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use futures::future::BoxFuture;
use futures::FutureExt;

use crate::error::{Error, Result};
use crate::message::{decode_message, encode_message, Message};
use crate::transport::{Client, Publisher, Service, Subscriber};

/// Type alias for service handler map
type ServiceHandlerMap = Arc<Mutex<HashMap<String, Box<dyn Fn(Vec<u8>) -> Vec<u8> + Send + Sync>>>>;

/// Mock transport for testing
pub struct MockTransport {
    /// Topics and their messages
    topics: Arc<Mutex<HashMap<String, Vec<Vec<u8>>>>>,
    /// Services and their handlers
    services: ServiceHandlerMap,
}

impl Default for MockTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl MockTransport {
    /// Creates a new mock transport
    pub fn new() -> Self {
        Self {
            topics: Arc::new(Mutex::new(HashMap::new())),
            services: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Creates a publisher for the given topic
    pub async fn create_publisher<M: Message>(&self, topic: &str) -> Result<MockPublisher<M>> {
        let topics = self.topics.clone();
        let topic_name = topic.to_string();

        // Ensure the topic exists
        {
            let mut topics_guard = topics.lock().unwrap();
            if !topics_guard.contains_key(&topic_name) {
                topics_guard.insert(topic_name.clone(), Vec::new());
            }
        }

        Ok(MockPublisher {
            topic: topic_name,
            topics,
            _phantom: PhantomData,
        })
    }

    /// Creates a subscriber for the given topic with a callback
    pub async fn create_subscriber<M: Message, F>(
        &self,
        topic: &str,
        callback: F,
    ) -> Result<MockSubscriber>
    where
        F: Fn(M) + Send + Sync + 'static,
    {
        let topics = self.topics.clone();
        let topic_name = topic.to_string();

        // Ensure the topic exists
        {
            let mut topics_guard = topics.lock().unwrap();
            if !topics_guard.contains_key(&topic_name) {
                topics_guard.insert(topic_name.clone(), Vec::new());
            }
        }

        // Create a thread that polls the topic for new messages
        let topic_name_clone = topic_name.clone();
        let topics_clone = topics.clone();

        // In a real implementation, we would spawn a thread here
        // For testing, we'll just process any existing messages
        let topics_guard = topics_clone.lock().unwrap();
        if let Some(messages) = topics_guard.get(&topic_name_clone) {
            for message_bytes in messages {
                if let Ok(message) = decode_message::<M>(message_bytes) {
                    callback(message);
                }
            }
        }

        Ok(MockSubscriber {
            topic: topic_name,
            _topics: topics,
        })
    }

    /// Creates a service for the given name with a handler
    pub async fn create_service<Req: Message, Res: Message, F>(
        &self,
        service_name: &str,
        handler: F,
    ) -> Result<MockService>
    where
        F: Fn(Req) -> Result<Res> + Send + Sync + 'static,
    {
        let services = self.services.clone();
        let service_name = service_name.to_string();

        // Create a handler that takes bytes and returns bytes
        let handler_wrapper = Box::new(move |request_bytes: Vec<u8>| {
            match decode_message::<Req>(&request_bytes) {
                Ok(request) => match handler(request) {
                    Ok(response) => encode_message(&response),
                    Err(_) => Vec::new(), // Empty response for error
                },
                Err(_) => Vec::new(), // Empty response for decoding error
            }
        });

        // Store the handler
        {
            let mut services_guard = services.lock().unwrap();
            services_guard.insert(service_name.clone(), handler_wrapper);
        }

        Ok(MockService {
            name: service_name,
            _services: services,
        })
    }

    /// Creates a client for the given service
    pub async fn create_client<Req: Message, Res: Message>(
        &self,
        service_name: &str,
    ) -> Result<MockClient<Req, Res>> {
        let services = self.services.clone();
        let service_name = service_name.to_string();

        Ok(MockClient {
            service_name,
            services,
            _phantom: PhantomData,
        })
    }
}

/// Mock publisher implementation
pub struct MockPublisher<M: Message> {
    /// Topic name
    topic: String,
    /// Topics and their messages
    topics: Arc<Mutex<HashMap<String, Vec<Vec<u8>>>>>,
    /// Phantom data for the message type
    _phantom: PhantomData<M>,
}

impl<M: Message> Publisher<M> for MockPublisher<M> {
    fn publish(&self, message: &M) -> Result<()> {
        let bytes = encode_message(message);
        let mut topics_guard = self.topics.lock().unwrap();
        if let Some(messages) = topics_guard.get_mut(&self.topic) {
            messages.push(bytes);
        }
        Ok(())
    }
}

/// Mock subscriber implementation
pub struct MockSubscriber {
    /// Topic name
    #[allow(dead_code)]
    topic: String,
    /// Topics and their messages
    _topics: Arc<Mutex<HashMap<String, Vec<Vec<u8>>>>>,
}

impl Subscriber for MockSubscriber {
    fn close(&self) -> Result<()> {
        // Nothing to do for mock
        Ok(())
    }
}

/// Mock service implementation
pub struct MockService {
    /// Service name
    #[allow(dead_code)]
    name: String,
    /// Services and their handlers
    _services: ServiceHandlerMap,
}

impl Service for MockService {
    fn close(&self) -> Result<()> {
        // Nothing to do for mock
        Ok(())
    }
}

/// Mock client implementation
pub struct MockClient<Req: Message, Res: Message> {
    /// Service name
    service_name: String,
    /// Services and their handlers
    services: ServiceHandlerMap,
    /// Phantom data for the request and response types
    _phantom: PhantomData<(Req, Res)>,
}

impl<Req: Message, Res: Message> Client<Req, Res> for MockClient<Req, Res> {
    fn call(&self, request: &Req) -> Result<Res> {
        let services_guard = self.services.lock().unwrap();
        if let Some(handler) = services_guard.get(&self.service_name) {
            let request_bytes = encode_message(request);
            let response_bytes = handler(request_bytes);
            if response_bytes.is_empty() {
                return Err(Error::ServiceCallFailed(self.service_name.clone()));
            }
            decode_message::<Res>(&response_bytes)
        } else {
            Err(Error::ServiceCallFailed(format!(
                "Service not found: {}",
                self.service_name
            )))
        }
    }

    fn call_async<'a>(&'a self, request: &'a Req) -> BoxFuture<'a, Result<Res>> {
        let result = self.call(request);
        async move { result }.boxed()
    }
}
