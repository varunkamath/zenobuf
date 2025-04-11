//! Parameter system for Zenobuf

use std::any::Any;
use std::sync::{Arc, Mutex};

use serde::{de::DeserializeOwned, Serialize};

use crate::error::{Error, Result};

/// Parameter for Zenobuf
///
/// A Parameter is a named value that can be set and retrieved.
pub struct Parameter {
    /// Name of the parameter
    name: String,
    /// Value of the parameter
    value: Arc<Mutex<Box<dyn Any + Send + Sync>>>,
    /// Serialized value of the parameter
    serialized: Arc<Mutex<String>>,
}

impl Parameter {
    /// Creates a new Parameter
    pub fn new<T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static>(
        name: &str,
        value: T,
    ) -> Result<Self> {
        let serialized = serde_json::to_string(&value).map_err(|e| {
            Error::Parameter(format!("Failed to serialize parameter {}: {}", name, e))
        })?;

        Ok(Self {
            name: name.to_string(),
            value: Arc::new(Mutex::new(Box::new(value))),
            serialized: Arc::new(Mutex::new(serialized)),
        })
    }

    /// Returns the name of the parameter
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the value of the parameter
    pub fn get_value<T: DeserializeOwned + Clone + Send + Sync + 'static>(&self) -> Result<T> {
        let value = self.value.lock().unwrap();
        
        // Try to downcast the value
        if let Some(typed_value) = value.downcast_ref::<T>() {
            return Ok(typed_value.clone());
        }
        
        // If downcasting fails, try to deserialize from the serialized value
        let serialized = self.serialized.lock().unwrap();
        let deserialized = serde_json::from_str::<T>(&serialized).map_err(|e| {
            Error::Parameter(format!(
                "Failed to deserialize parameter {}: {}",
                self.name, e
            ))
        })?;
        
        Ok(deserialized)
    }

    /// Sets the value of the parameter
    pub fn set_value<T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static>(
        &self,
        value: T,
    ) -> Result<()> {
        let serialized = serde_json::to_string(&value).map_err(|e| {
            Error::Parameter(format!("Failed to serialize parameter {}: {}", self.name, e))
        })?;
        
        *self.value.lock().unwrap() = Box::new(value);
        *self.serialized.lock().unwrap() = serialized;
        
        Ok(())
    }
}
