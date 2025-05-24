//! Node abstraction for Zenobuf

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::client::Client;
use crate::error::{Error, Result};
use crate::message::Message;
use crate::parameter::Parameter;
use crate::publisher::Publisher;
use crate::qos::{QosProfile, QosPreset};
use crate::service::Service;
use crate::subscriber::Subscriber;
use crate::transport::ZenohTransport;

/// A guard that automatically cleans up resources when dropped
pub struct DropGuard {
    cleanup: Box<dyn FnOnce() + Send + Sync>,
}

impl DropGuard {
    pub fn new<F>(cleanup: F) -> Self
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        Self {
            cleanup: Box::new(cleanup),
        }
    }
}

impl Drop for DropGuard {
    fn drop(&mut self) {
        // Take the cleanup function and call it
        let cleanup = std::mem::replace(&mut self.cleanup, Box::new(|| {}));
        cleanup();
    }
}

/// A handle to a publisher with automatic cleanup
pub struct PublisherHandle<M: Message> {
    publisher: Arc<Publisher<M>>,
    _cleanup: DropGuard,
}

impl<M: Message> PublisherHandle<M> {
    fn new(publisher: Arc<Publisher<M>>) -> Self {
        let cleanup = DropGuard::new(move || {
            // Cleanup logic can be added here if needed
        });

        Self {
            publisher,
            _cleanup: cleanup,
        }
    }

    /// Get the underlying publisher
    pub fn publisher(&self) -> &Arc<Publisher<M>> {
        &self.publisher
    }

    /// Publish a message
    pub fn publish(&self, message: &M) -> Result<()> {
        self.publisher.publish(message)
    }

    /// Get the topic name
    pub fn topic(&self) -> &str {
        self.publisher.topic()
    }
}

/// A handle to a subscriber with automatic cleanup
pub struct SubscriberHandle {
    subscriber: Arc<Subscriber>,
    _cleanup: DropGuard,
}

impl SubscriberHandle {
    fn new(subscriber: Arc<Subscriber>) -> Self {
        let cleanup = DropGuard::new(move || {
            // Cleanup logic can be added here if needed
        });

        Self {
            subscriber,
            _cleanup: cleanup,
        }
    }

    /// Get the underlying subscriber
    pub fn subscriber(&self) -> &Arc<Subscriber> {
        &self.subscriber
    }
}

/// A handle to a service with automatic cleanup
pub struct ServiceHandle {
    service: Arc<Service>,
    _cleanup: DropGuard,
}

impl ServiceHandle {
    fn new(service: Arc<Service>) -> Self {
        let cleanup = DropGuard::new(move || {
            // Cleanup logic can be added here if needed
        });

        Self {
            service,
            _cleanup: cleanup,
        }
    }

    /// Get the underlying service
    pub fn service(&self) -> &Arc<Service> {
        &self.service
    }
}

/// A handle to a client with automatic cleanup
pub struct ClientHandle<Req: Message, Res: Message> {
    client: Arc<Client<Req, Res>>,
    _cleanup: DropGuard,
}

impl<Req: Message, Res: Message> ClientHandle<Req, Res> {
    fn new(client: Arc<Client<Req, Res>>) -> Self {
        let cleanup = DropGuard::new(move || {
            // Cleanup logic can be added here if needed
        });

        Self {
            client,
            _cleanup: cleanup,
        }
    }

    /// Get the underlying client
    pub fn client(&self) -> &Arc<Client<Req, Res>> {
        &self.client
    }

    /// Call the service
    pub fn call(&self, request: &Req) -> Result<Res> {
        self.client.call(request)
    }

    /// Call the service asynchronously
    pub async fn call_async(&self, request: &Req) -> Result<Res> {
        self.client.call_async(request).await
    }
}

/// Node abstraction for Zenobuf
///
/// A Node is the main entry point for using Zenobuf. It provides methods for
/// creating publishers, subscribers, services, and clients.
pub struct Node {
    /// Name of the node
    name: String,
    /// Transport layer
    transport: ZenohTransport,
    /// Publishers
    publishers: Mutex<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>,
    /// Subscribers
    subscribers: Mutex<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>,
    /// Services
    services: Mutex<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>,
    /// Clients
    clients: Mutex<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>,
    /// Parameters
    parameters: Mutex<HashMap<String, Parameter>>,
}

impl Node {
    /// Creates a new Node with the given name
    pub async fn new(name: &str) -> Result<Self> {
        let transport = ZenohTransport::new().await?;
        Self::with_transport(name, transport)
    }

    /// Creates a new Node with the given name and transport
    pub fn with_transport(name: &str, transport: ZenohTransport) -> Result<Self> {
        Ok(Self {
            name: name.to_string(),
            transport,
            publishers: Mutex::new(HashMap::new()),
            subscribers: Mutex::new(HashMap::new()),
            services: Mutex::new(HashMap::new()),
            clients: Mutex::new(HashMap::new()),
            parameters: Mutex::new(HashMap::new()),
        })
    }

    /// Returns the name of the node
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Creates a publisher for the given topic
    pub async fn create_publisher<M: Message>(
        &self,
        topic: &str,
        _qos: QosProfile,
    ) -> Result<Arc<Publisher<M>>> {
        // Use the topic name as provided by the user (global topics by default)
        let topic_name = topic.to_string();

        // Check if the publisher already exists
        {
            let publishers = self.publishers.lock().unwrap();
            if publishers.contains_key(&topic_name) {
                return Err(Error::topic_already_exists(&topic_name, &self.name));
            }
        } // MutexGuard is dropped here

        // Create the publisher
        let inner_publisher = self.transport.create_publisher::<M>(&topic_name).await?;
        let publisher = Arc::new(Publisher::new(
            topic_name.clone(),
            Box::new(inner_publisher),
        ));

        // Store the publisher
        let mut publishers = self.publishers.lock().unwrap();
        publishers.insert(topic_name, Box::new(publisher.clone()));

        Ok(publisher)
    }

    /// Creates a subscriber for the given topic with a callback
    pub async fn create_subscriber<M: Message, F>(
        &self,
        topic: &str,
        _qos: QosProfile,
        callback: F,
    ) -> Result<Arc<Subscriber>>
    where
        F: Fn(M) + Send + Sync + 'static,
    {
        // Use the topic name as provided by the user (global topics by default)
        let topic_name = topic.to_string();

        // Check if the subscriber already exists
        {
            let subscribers = self.subscribers.lock().unwrap();
            if subscribers.contains_key(&topic_name) {
                return Err(Error::topic_already_exists(&topic_name, &self.name));
            }
        } // MutexGuard is dropped here

        // Create the subscriber
        let inner_subscriber = self
            .transport
            .create_subscriber::<M, F>(&topic_name, callback)
            .await?;
        let subscriber = Arc::new(Subscriber::new(
            topic_name.clone(),
            Box::new(inner_subscriber),
        ));

        // Store the subscriber
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.insert(topic_name, Box::new(subscriber.clone()));

        Ok(subscriber)
    }

    /// Creates a service for the given name with a handler
    pub async fn create_service<Req: Message, Res: Message, F>(
        &self,
        service_name: &str,
        handler: F,
    ) -> Result<Arc<Service>>
    where
        F: Fn(Req) -> Result<Res> + Send + Sync + 'static,
    {
        // Use the service name as provided by the user (global services by default)
        let full_service_name = service_name.to_string();

        // Check if the service already exists
        {
            let services = self.services.lock().unwrap();
            if services.contains_key(&full_service_name) {
                return Err(Error::service_already_exists(&full_service_name, &self.name));
            }
        } // MutexGuard is dropped here

        // Create the service
        let inner_service = self
            .transport
            .create_service::<Req, Res, F>(&full_service_name, handler)
            .await?;
        let service = Arc::new(Service::new(
            full_service_name.clone(),
            Box::new(inner_service),
        ));

        // Store the service
        let mut services = self.services.lock().unwrap();
        services.insert(full_service_name, Box::new(service.clone()));

        Ok(service)
    }

    /// Creates a client for the given service name
    pub fn create_client<Req: Message, Res: Message>(
        &self,
        service_name: &str,
    ) -> Result<Arc<Client<Req, Res>>> {
        // Use the service name as provided by the user (global services by default)
        let full_service_name = service_name.to_string();

        // Check if the client already exists
        let mut clients = self.clients.lock().unwrap();
        if clients.contains_key(&full_service_name) {
            return Err(Error::service_already_exists(&full_service_name, &self.name));
        }

        // Create the client
        let inner_client = self
            .transport
            .create_client::<Req, Res>(&full_service_name)?;
        let client = Arc::new(Client::new(
            full_service_name.clone(),
            Box::new(inner_client),
        ));

        // Store the client
        clients.insert(full_service_name, Box::new(client.clone()));

        Ok(client)
    }

    /// Sets a parameter
    pub fn set_parameter<
        T: serde::Serialize + serde::de::DeserializeOwned + Clone + Send + Sync + 'static,
    >(
        &self,
        name: &str,
        value: T,
    ) -> Result<()> {
        let mut parameters = self.parameters.lock().unwrap();
        parameters.insert(name.to_string(), Parameter::new(name, value)?);
        Ok(())
    }

    /// Gets a parameter
    pub fn get_parameter<T: serde::de::DeserializeOwned + Clone + Send + Sync + 'static>(
        &self,
        name: &str,
    ) -> Result<T> {
        let parameters = self.parameters.lock().unwrap();
        if let Some(param) = parameters.get(name) {
            param.get_value()
        } else {
            Err(Error::parameter(name, "Parameter not found"))
        }
    }

    /// Spins the node once, processing all pending callbacks
    pub fn spin_once(&self) -> Result<()> {
        // In a real implementation, this would process all pending callbacks
        // For now, we just return Ok
        Ok(())
    }

    /// Spins the node, processing callbacks until the node is shutdown
    pub async fn spin(&self) -> Result<()> {
        // In a real implementation, this would process callbacks until shutdown
        // For now, we just wait forever
        std::future::pending::<()>().await;
        Ok(())
    }

    // Builder pattern methods for simplified API

    /// Creates a publisher builder for the given topic
    pub fn publisher<M: Message>(&self, topic: &str) -> PublisherBuilder<M> {
        PublisherBuilder::new(self, topic)
    }

    /// Creates a subscriber builder for the given topic
    pub fn subscriber<M: Message>(&self, topic: &str) -> SubscriberBuilder<M> {
        SubscriberBuilder::new(self, topic)
    }

    /// Creates a service builder for the given service name
    pub fn service<Req: Message, Res: Message>(&self, name: &str) -> ServiceBuilder<Req, Res> {
        ServiceBuilder::new(self, name)
    }

    /// Creates a client builder for the given service name
    pub fn client<Req: Message, Res: Message>(&self, name: &str) -> ClientBuilder<Req, Res> {
        ClientBuilder::new(self, name)
    }

    // Simplified convenience methods

    /// Creates a publisher with default QoS
    pub async fn publish<M: Message>(&self, topic: &str) -> Result<Arc<Publisher<M>>> {
        self.create_publisher(topic, QosProfile::default()).await
    }

    /// Creates a subscriber with default QoS and a callback
    pub async fn subscribe<M: Message, F>(
        &self,
        topic: &str,
        callback: F,
    ) -> Result<Arc<Subscriber>>
    where
        F: Fn(M) + Send + Sync + 'static,
    {
        self.create_subscriber(topic, QosProfile::default(), callback)
            .await
    }
}

/// Builder for creating publishers with fluent API
pub struct PublisherBuilder<'a, M: Message> {
    node: &'a Node,
    topic: String,
    qos: QosProfile,
    _phantom: PhantomData<M>,
}

impl<'a, M: Message> PublisherBuilder<'a, M> {
    fn new(node: &'a Node, topic: &str) -> Self {
        Self {
            node,
            topic: topic.to_string(),
            qos: QosProfile::default(),
            _phantom: PhantomData,
        }
    }

    /// Sets the QoS profile
    pub fn with_qos(mut self, qos: QosProfile) -> Self {
        self.qos = qos;
        self
    }

    /// Sets the QoS preset
    pub fn with_qos_preset(mut self, preset: QosPreset) -> Self {
        self.qos = preset.into();
        self
    }

    /// Sets reliability
    pub fn reliable(mut self) -> Self {
        self.qos.reliability = crate::qos::Reliability::Reliable;
        self
    }

    /// Sets best effort reliability
    pub fn best_effort(mut self) -> Self {
        self.qos.reliability = crate::qos::Reliability::BestEffort;
        self
    }

    /// Sets the history depth
    pub fn with_depth(mut self, depth: usize) -> Self {
        self.qos.depth = depth;
        self
    }

    /// Builds the publisher
    pub async fn build(self) -> Result<PublisherHandle<M>> {
        let publisher = self.node.create_publisher(&self.topic, self.qos).await?;
        Ok(PublisherHandle::new(publisher))
    }
}

/// Builder for creating subscribers with fluent API
pub struct SubscriberBuilder<'a, M: Message> {
    node: &'a Node,
    topic: String,
    qos: QosProfile,
    _phantom: PhantomData<M>,
}

impl<'a, M: Message> SubscriberBuilder<'a, M> {
    fn new(node: &'a Node, topic: &str) -> Self {
        Self {
            node,
            topic: topic.to_string(),
            qos: QosProfile::default(),
            _phantom: PhantomData,
        }
    }

    /// Sets the QoS profile
    pub fn with_qos(mut self, qos: QosProfile) -> Self {
        self.qos = qos;
        self
    }

    /// Sets the QoS preset
    pub fn with_qos_preset(mut self, preset: QosPreset) -> Self {
        self.qos = preset.into();
        self
    }

    /// Sets reliability
    pub fn reliable(mut self) -> Self {
        self.qos.reliability = crate::qos::Reliability::Reliable;
        self
    }

    /// Sets best effort reliability
    pub fn best_effort(mut self) -> Self {
        self.qos.reliability = crate::qos::Reliability::BestEffort;
        self
    }

    /// Sets the history depth
    pub fn with_depth(mut self, depth: usize) -> Self {
        self.qos.depth = depth;
        self
    }

    /// Builds the subscriber with a callback
    pub async fn build<F>(self, callback: F) -> Result<SubscriberHandle>
    where
        F: Fn(M) + Send + Sync + 'static,
    {
        let subscriber = self.node
            .create_subscriber(&self.topic, self.qos, callback)
            .await?;
        Ok(SubscriberHandle::new(subscriber))
    }
}

/// Builder for creating services with fluent API
pub struct ServiceBuilder<'a, Req: Message, Res: Message> {
    node: &'a Node,
    name: String,
    _phantom: PhantomData<(Req, Res)>,
}

impl<'a, Req: Message, Res: Message> ServiceBuilder<'a, Req, Res> {
    fn new(node: &'a Node, name: &str) -> Self {
        Self {
            node,
            name: name.to_string(),
            _phantom: PhantomData,
        }
    }

    /// Builds the service with a handler
    pub async fn build<F>(self, handler: F) -> Result<ServiceHandle>
    where
        F: Fn(Req) -> Result<Res> + Send + Sync + 'static,
    {
        let service = self.node.create_service(&self.name, handler).await?;
        Ok(ServiceHandle::new(service))
    }
}

/// Builder for creating clients with fluent API
pub struct ClientBuilder<'a, Req: Message, Res: Message> {
    node: &'a Node,
    name: String,
    _phantom: PhantomData<(Req, Res)>,
}

impl<'a, Req: Message, Res: Message> ClientBuilder<'a, Req, Res> {
    fn new(node: &'a Node, name: &str) -> Self {
        Self {
            node,
            name: name.to_string(),
            _phantom: PhantomData,
        }
    }

    /// Builds the client
    pub fn build(self) -> Result<ClientHandle<Req, Res>> {
        let client = self.node.create_client(&self.name)?;
        Ok(ClientHandle::new(client))
    }
}
