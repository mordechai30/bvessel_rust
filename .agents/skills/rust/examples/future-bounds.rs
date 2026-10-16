//! Make the returned future's Send promise explicit at a public trait boundary.
//! A transferable callable or Sync receiver does not promise a transferable future.
//!
//! This generic caller cannot infer Send from an async trait method:
//! ```compile_fail,E0277
//! use std::future::Future;
//! trait Fetch: Sync { async fn fetch(&self) -> usize; }
//! fn require_send(_: impl Future + Send) {}
//! fn caller<T: Fetch>(value: &T) { require_send(value.fetch()); }
//! ```
//!
//! Promise Send on the returned future when callers require it:
//! ```
//! use std::future::Future;
//! trait Fetch: Sync {
//!     fn fetch(&self) -> impl Future<Output = usize> + Send;
//! }
//! fn require_send(_: impl Future + Send) {}
//! fn caller<T: Fetch>(value: &T) { require_send(value.fetch()); }
//! ```
//!
//! Borrowing an owned capture across await can still produce a Send future:
//! ```
//! use std::future::Future;
//! use std::sync::Arc;
//! async fn inspect(value: &str) -> usize { value.len() }
//! fn require_send(_: impl Future<Output = usize> + Send + 'static) {}
//! let value: Arc<str> = Arc::from("retained");
//! require_send(async move { inspect(&value).await });
//! ```

use std::future::Future;

/// Supplies owned data through a future that callers can transfer between threads.
/// Implementations must keep captured and suspended state compatible with Send.
pub trait Fetch: Sync {
    /// Creates one borrowing operation without imposing a static lifetime.
    /// The caller must keep the receiver live until this operation ends.
    fn fetch(&self) -> impl Future<Output = usize> + Send;
}
