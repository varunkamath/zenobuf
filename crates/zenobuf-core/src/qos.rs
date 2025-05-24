//! Quality of Service (QoS) profiles for Zenobuf

use std::time::Duration;

/// QoS preset for common use cases
///
/// This enum provides convenient presets for common QoS configurations,
/// making it easier to configure quality of service without verbose setup.
#[derive(Debug, Clone)]
pub enum QosPreset {
    /// Default QoS profile - reliable, volatile, keep last 10
    Default,
    /// Optimized for sensor data - best effort, volatile, keep last 5
    SensorData,
    /// Optimized for parameters - reliable, transient local, keep last 1
    Parameters,
    /// Optimized for services - reliable, volatile, keep last 10, 1s deadline
    Services,
    /// High throughput - best effort, volatile, keep last 100
    HighThroughput,
    /// Low latency - best effort, volatile, keep last 1
    LowLatency,
    /// Custom QoS profile
    Custom(QosProfile),
}

/// Quality of Service profile for publishers and subscribers
///
/// This struct defines the quality of service parameters for publishers and
/// subscribers. It is similar to the QoS profiles in ROS.
#[derive(Debug, Clone)]
pub struct QosProfile {
    /// Reliability of the communication
    pub reliability: Reliability,
    /// Durability of the communication
    pub durability: Durability,
    /// History policy
    pub history: History,
    /// Depth of the history queue
    pub depth: usize,
    /// Deadline for receiving messages
    pub deadline: Option<Duration>,
    /// Lifespan of messages
    pub lifespan: Option<Duration>,
}

impl Default for QosProfile {
    fn default() -> Self {
        Self {
            reliability: Reliability::Reliable,
            durability: Durability::Volatile,
            history: History::KeepLast,
            depth: 10,
            deadline: None,
            lifespan: None,
        }
    }
}

impl QosProfile {
    /// Creates a new QoS profile with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the reliability of the QoS profile
    pub fn reliability(mut self, reliability: Reliability) -> Self {
        self.reliability = reliability;
        self
    }

    /// Sets the durability of the QoS profile
    pub fn durability(mut self, durability: Durability) -> Self {
        self.durability = durability;
        self
    }

    /// Sets the history policy of the QoS profile
    pub fn history(mut self, history: History) -> Self {
        self.history = history;
        self
    }

    /// Sets the depth of the history queue
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    /// Sets the deadline for receiving messages
    pub fn deadline(mut self, deadline: Duration) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Sets the lifespan of messages
    pub fn lifespan(mut self, lifespan: Duration) -> Self {
        self.lifespan = Some(lifespan);
        self
    }

    /// Creates a QoS profile for sensors
    ///
    /// This profile is optimized for sensor data, which is typically
    /// high-frequency and where the latest data is most important.
    pub fn sensor_data() -> Self {
        Self {
            reliability: Reliability::BestEffort,
            durability: Durability::Volatile,
            history: History::KeepLast,
            depth: 5,
            deadline: None,
            lifespan: None,
        }
    }

    /// Creates a QoS profile for parameters
    ///
    /// This profile is optimized for parameters, which are typically
    /// low-frequency and where reliability is important.
    pub fn parameters() -> Self {
        Self {
            reliability: Reliability::Reliable,
            durability: Durability::TransientLocal,
            history: History::KeepLast,
            depth: 1,
            deadline: None,
            lifespan: None,
        }
    }

    /// Creates a QoS profile for services
    ///
    /// This profile is optimized for services, which require reliable
    /// communication.
    pub fn services() -> Self {
        Self {
            reliability: Reliability::Reliable,
            durability: Durability::Volatile,
            history: History::KeepLast,
            depth: 10,
            deadline: Some(Duration::from_secs(1)),
            lifespan: None,
        }
    }
}

/// Reliability of the communication
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reliability {
    /// Best effort delivery (may drop messages)
    BestEffort,
    /// Reliable delivery (guaranteed delivery)
    Reliable,
}

/// Durability of the communication
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Durability {
    /// Volatile durability (no persistence)
    Volatile,
    /// Transient local durability (persistence on the publisher side)
    TransientLocal,
}

/// History policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum History {
    /// Keep the last N messages
    KeepLast,
    /// Keep all messages
    KeepAll,
}

impl From<QosPreset> for QosProfile {
    fn from(preset: QosPreset) -> Self {
        match preset {
            QosPreset::Default => QosProfile::default(),
            QosPreset::SensorData => QosProfile::sensor_data(),
            QosPreset::Parameters => QosProfile::parameters(),
            QosPreset::Services => QosProfile::services(),
            QosPreset::HighThroughput => QosProfile {
                reliability: Reliability::BestEffort,
                durability: Durability::Volatile,
                history: History::KeepLast,
                depth: 100,
                deadline: None,
                lifespan: None,
            },
            QosPreset::LowLatency => QosProfile {
                reliability: Reliability::BestEffort,
                durability: Durability::Volatile,
                history: History::KeepLast,
                depth: 1,
                deadline: None,
                lifespan: None,
            },
            QosPreset::Custom(profile) => profile,
        }
    }
}

impl Default for QosPreset {
    fn default() -> Self {
        QosPreset::Default
    }
}
