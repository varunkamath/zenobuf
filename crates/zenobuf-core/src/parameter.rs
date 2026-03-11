//! Parameter system for Zenobuf

use std::any::Any;
use std::sync::Mutex;

use serde::{de::DeserializeOwned, Serialize};

use crate::error::{Error, Result};

/// Parameter for Zenobuf
///
/// A Parameter is a named value that can be set and retrieved.
pub struct Parameter {
    /// Name of the parameter
    name: String,
    /// Value and its serialized form, kept in sync under a single lock
    inner: Mutex<(Box<dyn Any + Send + Sync>, String)>,
}

impl Parameter {
    /// Creates a new Parameter
    pub fn new<T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static>(
        name: &str,
        value: T,
    ) -> Result<Self> {
        let serialized = serde_json::to_string(&value)
            .map_err(|e| Error::parameter(name, format!("Failed to serialize: {e}")))?;

        Ok(Self {
            name: name.to_string(),
            inner: Mutex::new((Box::new(value), serialized)),
        })
    }

    /// Returns the name of the parameter
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the value of the parameter
    pub fn get_value<T: DeserializeOwned + Clone + Send + Sync + 'static>(&self) -> Result<T> {
        let guard = self.inner.lock().unwrap();
        let (ref value, ref serialized) = *guard;

        // Try to downcast the value
        if let Some(typed_value) = value.downcast_ref::<T>() {
            return Ok(typed_value.clone());
        }

        // If downcasting fails, try to deserialize from the serialized value
        let deserialized = serde_json::from_str::<T>(serialized)
            .map_err(|e| Error::parameter(&self.name, format!("Failed to deserialize: {e}")))?;

        Ok(deserialized)
    }

    /// Sets the value of the parameter
    pub fn set_value<T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static>(
        &self,
        value: T,
    ) -> Result<()> {
        let serialized = serde_json::to_string(&value)
            .map_err(|e| Error::parameter(&self.name, format!("Failed to serialize: {e}")))?;

        let mut guard = self.inner.lock().unwrap();
        *guard = (Box::new(value), serialized);

        Ok(())
    }
}
