//! Callback executor for processing subscriber callbacks
//!
//! This module provides a simple callback queue that allows subscribers to enqueue
//! callbacks for later processing by the node's spin methods.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// A callback that can be executed by the executor
pub type Callback = Box<dyn FnOnce() + Send>;

/// A simple executor that queues callbacks for later processing
///
/// The executor provides a thread-safe way to enqueue callbacks from subscriber
/// threads and process them in the node's spin loop.
#[derive(Clone)]
pub struct CallbackExecutor {
    callbacks: Arc<Mutex<VecDeque<Callback>>>,
    shutdown: Arc<AtomicBool>,
    notify: Arc<tokio::sync::Notify>,
}

impl Default for CallbackExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl CallbackExecutor {
    /// Creates a new callback executor
    pub fn new() -> Self {
        Self {
            callbacks: Arc::new(Mutex::new(VecDeque::new())),
            shutdown: Arc::new(AtomicBool::new(false)),
            notify: Arc::new(tokio::sync::Notify::new()),
        }
    }

    /// Enqueues a callback for later processing
    ///
    /// This method is thread-safe and can be called from subscriber callbacks.
    pub fn enqueue(&self, callback: Callback) {
        if !self.is_shutdown() {
            self.callbacks.lock().unwrap().push_back(callback);
            self.notify.notify_one();
        }
    }

    /// Processes all pending callbacks
    ///
    /// Returns the number of callbacks that were processed.
    pub fn process_pending(&self) -> usize {
        let callbacks: Vec<Callback> = {
            let mut queue = self.callbacks.lock().unwrap();
            queue.drain(..).collect()
        };

        let count = callbacks.len();
        for callback in callbacks {
            callback();
        }

        count
    }

    /// Returns the number of pending callbacks
    pub fn pending_count(&self) -> usize {
        self.callbacks.lock().unwrap().len()
    }

    /// Returns a future that resolves when a callback is enqueued or shutdown is signaled.
    /// Must be called (to register interest) before draining the queue.
    pub fn notified(&self) -> tokio::sync::futures::Notified<'_> {
        self.notify.notified()
    }

    /// Signals the executor to shutdown
    ///
    /// After shutdown, no new callbacks will be accepted.
    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
        self.notify.notify_one();
    }

    /// Returns true if the executor has been shutdown
    pub fn is_shutdown(&self) -> bool {
        self.shutdown.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn test_enqueue_and_process() {
        let executor = CallbackExecutor::new();
        let counter = Arc::new(AtomicUsize::new(0));

        // Enqueue some callbacks
        for _ in 0..5 {
            let counter = counter.clone();
            executor.enqueue(Box::new(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            }));
        }

        assert_eq!(executor.pending_count(), 5);

        // Process callbacks
        let processed = executor.process_pending();

        assert_eq!(processed, 5);
        assert_eq!(counter.load(Ordering::SeqCst), 5);
        assert_eq!(executor.pending_count(), 0);
    }

    #[test]
    fn test_shutdown() {
        let executor = CallbackExecutor::new();
        let counter = Arc::new(AtomicUsize::new(0));

        // Enqueue before shutdown
        let counter1 = counter.clone();
        executor.enqueue(Box::new(move || {
            counter1.fetch_add(1, Ordering::SeqCst);
        }));

        // Shutdown
        executor.shutdown();
        assert!(executor.is_shutdown());

        // Enqueue after shutdown should be ignored
        let counter2 = counter.clone();
        executor.enqueue(Box::new(move || {
            counter2.fetch_add(1, Ordering::SeqCst);
        }));

        // Only the first callback should be in the queue
        assert_eq!(executor.pending_count(), 1);

        // Process should still work for queued callbacks
        executor.process_pending();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_clone() {
        let executor = CallbackExecutor::new();
        let executor2 = executor.clone();
        let counter = Arc::new(AtomicUsize::new(0));

        // Enqueue via first executor
        let counter1 = counter.clone();
        executor.enqueue(Box::new(move || {
            counter1.fetch_add(1, Ordering::SeqCst);
        }));

        // Process via second executor
        let processed = executor2.process_pending();

        assert_eq!(processed, 1);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
