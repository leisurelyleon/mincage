//! Which Linux namespaces a container isolates — expressed as pure data.
//!
//! This module knows nothing about syscalls; the runtime layer maps these
//! values onto kernel clone flags.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// A single Linux namespace kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Namespace {
    Pid,
    Mount,
    Uts,
    Network,
    User,
    Ipc,
}

/// The set of namespaces a container isolates.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamespaceSet {
    enabled: BTreeSet<Namespace>,
}

impl NamespaceSet {
    /// An empty set (no isolation).
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder-style insertion.
    #[must_use]
    pub fn with(mut self, namespace: Namespace) -> Self {
        self.enabled.insert(namespace);
        self
    }

    /// The conventional default isolation: PID, mount, and UTS namespaces.
    pub fn default_isolation() -> Self {
        Self::new()
            .with(Namespace::Pid)
            .with(Namespace::Mount)
            .with(Namespace::Uts)
    }

    pub fn contains(&self, namespace: Namespace) -> bool {
        self.enabled.contains(&namespace)
    }

    pub fn is_empty(&self) -> bool {
        self.enabled.is_empty()
    }

    /// Iterates the enabled namespaces in a stable order.
    pub fn iter(&self) -> impl Iterator<Item = Namespace> + '_ {
        self.enabled.iter().copied()
    }
}

impl FromIterator<Namespace> for NamespaceSet {
    fn from_iter<I: IntoIterator<Item = Namespace>>(iter: I) -> Self {
        Self {
            enabled: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_isolation_has_pid_mount_uts_only() {
        let set = NamespaceSet::default_isolation();
        assert!(set.contains(Namespace::Pid));
        assert!(set.contains(Namespace::Mount));
        assert!(set.contains(Namespace::Uts));
        assert!(!set.contains(Namespace::Network));
    }

    #[test]
    fn from_iter_deduplicates() {
        let set: NamespaceSet = [Namespace::Pid, Namespace::Pid, Namespace::Uts]
            .into_iter()
            .collect();
        assert_eq!(set.iter().count(), 2);
    }

    #[test]
    fn empty_set_reports_empty() {
        assert!(NamespaceSet::new().is_empty());
    }
}
