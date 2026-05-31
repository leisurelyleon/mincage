//! Pure container-specification logic for `mincage`.
//!
//! This crate is intentionally syscall-free: it models *what* a container is
//! (namespaces, cgroup limits, mounts, command) as data, and validates it. The
//! `mincage-runtime` crate maps these values onto real Linux syscalls.

pub mod cgroup_spec;
pub mod config;
pub mod error;
pub mod mount_plan;
pub mod namespaces;

pub use cgroup_spec::CgroupSpec;
pub use config::{ContainerConfig, SAMPLE_SPEC};
pub use error::CoreError;
pub use mount_plan::{MountFlag, MountOp, MountPlan};
pub use namespaces::{Namespace, NamespaceSet};
