/// Backpressure handling utilities for async I/O operations
///
/// This module provides utilities for managing backpressure in async I/O scenarios,
/// ensuring that the system doesn't get overwhelmed by too many concurrent operations.

#[cfg(feature = "async")]
use std::sync::Arc;
#[cfg(feature = "async")]
use tokio::sync::Semaphore;

/// Backpressure controller to limit concurrent I/O operations
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct BackpressureController {
	semaphore: Arc<Semaphore>,
	max_concurrent: usize,
}

#[cfg(feature = "async")]
impl BackpressureController {
	/// Create a new backpressure controller with a maximum number of concurrent operations
	pub fn new(max_concurrent: usize) -> Self {
		Self {
			semaphore: Arc::new(Semaphore::new(max_concurrent)),
			max_concurrent,
		}
	}

	/// Acquire a permit to perform an I/O operation
	/// This will block if the maximum number of concurrent operations is reached
	pub async fn acquire(&self) -> BackpressurePermit {
		let permit = self
			.semaphore
			.clone()
			.acquire_owned()
			.await
			.expect("Semaphore should not be closed");
		BackpressurePermit { _permit: permit }
	}

	/// Try to acquire a permit without blocking
	/// Returns None if the maximum number of concurrent operations is reached
	pub fn try_acquire(&self) -> Option<BackpressurePermit> {
		self.semaphore
			.clone()
			.try_acquire_owned()
			.ok()
			.map(|permit| BackpressurePermit { _permit: permit })
	}

	/// Get the number of available permits
	pub fn available_permits(&self) -> usize {
		self.semaphore.available_permits()
	}

	/// Get the maximum number of concurrent operations
	pub fn max_concurrent(&self) -> usize {
		self.max_concurrent
	}
}

/// A permit to perform an I/O operation
/// The permit is automatically released when dropped
#[cfg(feature = "async")]
pub struct BackpressurePermit {
	_permit: tokio::sync::OwnedSemaphorePermit,
}

/// Default backpressure controller for typical workloads
#[cfg(feature = "async")]
impl Default for BackpressureController {
	fn default() -> Self {
		// Default to 100 concurrent I/O operations
		Self::new(100)
	}
}

#[cfg(all(test, feature = "async"))]
mod tests {
	use super::*;
	use std::time::Duration;

	#[tokio::test]
	async fn test_backpressure_controller() {
		let controller = BackpressureController::new(2);
		assert_eq!(controller.max_concurrent(), 2);
		assert_eq!(controller.available_permits(), 2);

		let _permit1 = controller.acquire().await;
		assert_eq!(controller.available_permits(), 1);

		let _permit2 = controller.acquire().await;
		assert_eq!(controller.available_permits(), 0);

		// Try to acquire should fail now
		assert!(controller.try_acquire().is_none());
	}

	#[tokio::test]
	async fn test_backpressure_release() {
		let controller = BackpressureController::new(1);

		{
			let _permit = controller.acquire().await;
			assert_eq!(controller.available_permits(), 0);
		}

		// Permit should be released
		assert_eq!(controller.available_permits(), 1);
	}

	#[tokio::test]
	async fn test_backpressure_concurrent() {
		let controller = BackpressureController::new(2);
		let controller_clone1 = controller.clone();
		let controller_clone2 = controller.clone();
		let controller_clone3 = controller.clone();

		// Spawn 3 tasks, but only 2 can run concurrently
		let task1 = tokio::spawn(async move {
			let _permit = controller_clone1.acquire().await;
			tokio::time::sleep(Duration::from_millis(50)).await;
			"task1"
		});

		let task2 = tokio::spawn(async move {
			let _permit = controller_clone2.acquire().await;
			tokio::time::sleep(Duration::from_millis(50)).await;
			"task2"
		});

		let task3 = tokio::spawn(async move {
			// Small delay to ensure task1 and task2 acquire first
			tokio::time::sleep(Duration::from_millis(10)).await;
			let _permit = controller_clone3.acquire().await;
			"task3"
		});

		let results = tokio::try_join!(task1, task2, task3);
		assert!(results.is_ok());
	}

	#[tokio::test]
	async fn test_default_controller() {
		let controller = BackpressureController::default();
		assert_eq!(controller.max_concurrent(), 100);
		assert_eq!(controller.available_permits(), 100);
	}
}
