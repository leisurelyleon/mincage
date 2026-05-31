//! The privileged Linux isolation layer for `mincage`.
//!
//! Pure orchestration lives in `Container`, syscalls behind the
//! `IsolationBackend` trait. `LinuxBackend` performs the real work;
//! `NoopBackend` records calls for testing.

pub mod backend;
pub mod container;
pub mod error;
pub mod linux_backend;

pub use backend::{IsolationBackend, NoopBackend};
pub use container::{launch, Container};
pub use error::{RuntimeError, RuntimeResult};
pub use linux_backend::LinuxBackend;
