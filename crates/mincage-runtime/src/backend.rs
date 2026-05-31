//! The isolation backend trait — the seam between pure orchestration and real
//! privileged syscalls. A `NoopBackend` records calls for testing.

use std::cell::RefCell;
use std::path::Path;

use mincage_core::{CgroupSpec, MountPlan, NamespaceSet};

use crate::error::RuntimeResult;

/// Abstraction over the privileged operations a container launch performs.
pub trait IsolationBackend {
    fn unshare(&self, namespaces: &NamespaceSet) -> RuntimeResult<()>;
    fn set_hostname(&self, name: &str) -> RuntimeResult<()>;
    fn pivot_into(&self, rootfs: &Path) -> RuntimeResult<()>;
    fn apply_mounts(&self, plan: &MountPlan) -> RuntimeResult<()>;
    fn apply_cgroups(&self, name: &str, spec: &CgroupSpec, pid: i32) -> RuntimeResult<()>;
}

/// A backend that performs no real operations and records the calls made.
/// Used to test orchestration order without any privileges.
#[derive(Debug, Default)]
pub struct NoopBackend {
    calls: RefCell<Vec<String>>,
}

impl NoopBackend {
    pub fn new() -> Self {
        Self::default()
    }

    /// The recorded call log, in order.
    pub fn calls(&self) -> Vec<String> {
        self.calls.borrow().clone()
    }

    fn record(&self, entry: String) {
        self.calls.borrow_mut().push(entry);
    }
}

impl IsolationBackend for NoopBackend {
    fn unshare(&self, _namespaces: &NamespaceSet) -> RuntimeResult<()> {
        self.record("unshare".to_string());
        Ok(())
    }

    fn set_hostname(&self, name: &str) -> RuntimeResult<()> {
        self.record(format!("set_hostname:{name}"));
        Ok(())
    }

    fn pivot_into(&self, rootfs: &Path) -> RuntimeResult<()> {
        self.record(format!("pivot_into:{}", rootfs.display()));
        Ok(())
    }

    fn apply_mounts(&self, plan: &MountPlan) -> RuntimeResult<()> {
        self.record(format!("apply_mounts:{}", plan.ops().len()));
        Ok(())
    }

    fn apply_cgroups(&self, _name: &str, _spec: &CgroupSpec, _pid: i32) -> RuntimeResult<()> {
        self.record("apply_cgroups".to_string());
        Ok(())
    }
}
