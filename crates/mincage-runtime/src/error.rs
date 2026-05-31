//! Runtime error type spanning syscalls, I/O, and core validation.

/// Errors from the privileged runtime layer.
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("syscall failed: {0}")]
    Syscall(#[from] nix::errno::Errno),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid C string: {0}")]
    Nul(#[from] std::ffi::NulError),

    #[error("core error: {0}")]
    Core(#[from] mincage_core::CoreError),
}

/// Convenience result alias.
pub type RuntimeResult<T> = std::result::Result<T, RuntimeError>;
