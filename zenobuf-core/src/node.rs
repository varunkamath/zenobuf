//! Node abstraction for Zenobuf

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::client::Client;
use crate::error::{Error, Result};
use crate::message::Message;
use crate::parameter::Parameter;
use crate::publisher::Publisher;
use crate::qos::QosProfile;
use crate::service::Service;
use crate::subscriber::Subscriber;
use crate::transport::ZenohTransport;

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
        let mut publishers = self.publishers.lock().unwrap();
        if publishers.contains_key(&topic_name) {
            return Err(Error::TopicAlreadyExists(topic_name));
        }

        // Create the publisher
        let inner_publisher = self.transport.create_publisher::<M>(&topic_name).await?;
        let publisher = Arc::new(Publisher::new(
            topic_name.clone(),
            Box::new(inner_publisher),
        ));

        // Store the publisher
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
        let mut subscribers = self.subscribers.lock().unwrap();
        if subscribers.contains_key(&topic_name) {
            return Err(Error::TopicAlreadyExists(topic_name));
        }

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
        let mut services = self.services.lock().unwrap();
        if services.contains_key(&full_service_name) {
            return Err(Error::ServiceAlreadyExists(full_service_name));
        }

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
            return Err(Error::ServiceAlreadyExists(full_service_name));
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
            Err(Error::Parameter(format!("Parameter not found: {}", name)))
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
}
