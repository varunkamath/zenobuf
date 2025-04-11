//! Time utilities for Zenobuf

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Time representation for Zenobuf
///
/// This struct represents a point in time, similar to the Time message in ROS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    /// Seconds since the Unix epoch
    pub sec: u64,
    /// Nanoseconds since the last second
    pub nsec: u32,
}

impl Time {
    /// Creates a new Time from seconds and nanoseconds
    pub fn new(sec: u64, nsec: u32) -> Self {
        let mut time = Self { sec, nsec };
        time.normalize();
        time
    }

    /// Creates a Time representing the current time
    pub fn now() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before Unix epoch");
        Self::from(now)
    }

    /// Creates a Time from a Duration since the Unix epoch
    pub fn from_duration(duration: Duration) -> Self {
        Self::new(duration.as_secs(), duration.subsec_nanos())
    }

    /// Converts the Time to a Duration since the Unix epoch
    pub fn to_duration(&self) -> Duration {
        Duration::new(self.sec, self.nsec)
    }

    /// Normalizes the Time by carrying over nanoseconds to seconds
    fn normalize(&mut self) {
        if self.nsec >= 1_000_000_000 {
            self.sec += u64::from(self.nsec) / 1_000_000_000;
            self.nsec %= 1_000_000_000;
        }
    }

    /// Adds a Duration to the Time
    pub fn add(&self, duration: Duration) -> Self {
        let duration_since_epoch = self.to_duration();
        let new_duration = duration_since_epoch + duration;
        Self::from_duration(new_duration)
    }

    /// Subtracts a Duration from the Time
    pub fn sub(&self, duration: Duration) -> Self {
        let duration_since_epoch = self.to_duration();
        let new_duration = duration_since_epoch
            .checked_sub(duration)
            .unwrap_or_else(|| Duration::new(0, 0));
        Self::from_duration(new_duration)
    }
}

impl From<Duration> for Time {
    fn from(duration: Duration) -> Self {
        Self::from_duration(duration)
    }
}

impl From<SystemTime> for Time {
    fn from(time: SystemTime) -> Self {
        let duration = time
            .duration_since(UNIX_EPOCH)
            .expect("System time before Unix epoch");
        Self::from(duration)
    }
}

/// Duration representation for Zenobuf
///
/// This struct represents a duration, similar to the Duration message in ROS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZenobufDuration {
    /// Seconds
    pub sec: i32,
    /// Nanoseconds
    pub nsec: i32,
}

impl ZenobufDuration {
    /// Creates a new Duration from seconds and nanoseconds
    pub fn new(sec: i32, nsec: i32) -> Self {
        let mut duration = Self { sec, nsec };
        duration.normalize();
        duration
    }

    /// Creates a Duration from a std::time::Duration
    pub fn from_std(duration: Duration) -> Self {
        Self::new(duration.as_secs() as i32, duration.subsec_nanos() as i32)
    }

    /// Converts the Duration to a std::time::Duration
    pub fn to_std(&self) -> Duration {
        Duration::new(self.sec as u64, self.nsec as u32)
    }

    /// Normalizes the Duration by carrying over nanoseconds to seconds
    fn normalize(&mut self) {
        if self.nsec >= 1_000_000_000 {
            self.sec += self.nsec / 1_000_000_000;
            self.nsec %= 1_000_000_000;
        } else if self.nsec < 0 {
            let sec_adj = (-self.nsec / 1_000_000_000) + 1;
            self.sec -= sec_adj;
            self.nsec += sec_adj * 1_000_000_000;
        }
    }
}

impl From<Duration> for ZenobufDuration {
    fn from(duration: Duration) -> Self {
        Self::from_std(duration)
    }
}

impl From<ZenobufDuration> for Duration {
    fn from(duration: ZenobufDuration) -> Self {
        duration.to_std()
    }
}
