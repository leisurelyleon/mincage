//! Mount operations expressed as pure, ordered data.

use serde::{Deserialize, Serialize};

/// A mount flag, mapped to a kernel flag by the runtime layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MountFlag {
    Bind,
    Rec,
    Private,
    NoSuid,
    NoDev,
    NoExec,
    ReadOnly,
}

/// A single mount operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MountOp {
    pub source: Option<String>,
    pub target: String,
    pub fstype: Option<String>,
    pub flags: Vec<MountFlag>,
}

impl MountOp {
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn fstype(&self) -> Option<&str> {
        self.fstype.as_deref()
    }

    pub fn flags(&self) -> &[MountFlag] {
        &self.flags
    }
}

/// An ordered plan of mount operations performed inside the container.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MountPlan {
    ops: Vec<MountOp>,
}

impl MountPlan {
    pub fn new(ops: Vec<MountOp>) -> Self {
        Self { ops }
    }

    /// The standard minimal plan: a fresh `/proc` for the new PID namespace.
    pub fn standard() -> Self {
        Self::new(vec![MountOp {
            source: Some("proc".to_string()),
            target: "/proc".to_string(),
            fstype: Some("proc".to_string()),
            flags: vec![MountFlag::NoSuid, MountFlag::NoDev, MountFlag::NoExec],
        }])
    }

    pub fn ops(&self) -> &[MountOp] {
        &self.ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_plan_mounts_proc() {
        let plan = MountPlan::standard();
        assert_eq!(plan.ops().len(), 1);
        let proc = &plan.ops()[0];
        assert_eq!(proc.target(), "/proc");
        assert_eq!(proc.fstype(), Some("proc"));
        assert!(proc.flags().contains(&MountFlag::NoSuid));
    }
}
