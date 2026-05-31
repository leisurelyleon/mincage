//! The container specification, loadable from a TOML spec file.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::cgroup_spec::CgroupSpec;
use crate::error::{CoreError, Result};
use crate::namespaces::{Namespace, NamespaceSet};

/// A sample container specification, printed by the CLI's `spec` command.
pub const SAMPLE_SPEC: &str = r#"name = "demo"
command = "/bin/sh"
args = ["-c", "echo hello from mincage; hostname; id"]
hostname = "mincage"
rootfs = "/"
namespaces = ["pid", "mount", "uts"]

[cgroups]
memory_max_bytes = 134217728
cpu_quota_us = 50000
cpu_period_us = 100000
"#;

fn default_hostname() -> String {
    "mincage".to_string()
}

fn default_rootfs() -> PathBuf {
    PathBuf::from("/")
}

/// A full container specification.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContainerConfig {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default = "default_hostname")]
    pub hostname: String,
    #[serde(default = "default_rootfs")]
    pub rootfs: PathBuf,
    #[serde(default)]
    pub namespaces: Vec<Namespace>,
    #[serde(default)]
    pub cgroups: CgroupSpec,
}

impl ContainerConfig {
    /// Validates the specification. Pure: performs no filesystem checks.
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(CoreError::Validation("name must not be empty".to_string()));
        }
        if self.command.trim().is_empty() {
            return Err(CoreError::Validation("command must not be empty".to_string()));
        }
        Ok(())
    }

    /// The namespaces to isolate, as a set.
    pub fn namespace_set(&self) -> NamespaceSet {
        self.namespaces.iter().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sample_spec() {
        let config: ContainerConfig = toml::from_str(SAMPLE_SPEC).unwrap();
        assert_eq!(config.name, "demo");
        assert_eq!(config.command, "/bin/sh");
        assert_eq!(config.hostname, "mincage");
        assert!(config.namespace_set().contains(Namespace::Pid));
        assert_eq!(config.cgroups.memory_max_bytes, Some(134_217_728));
    }

    #[test]
    fn validate_rejects_empty_command() {
        let config = ContainerConfig {
            name: "x".to_string(),
            command: "  ".to_string(),
            args: vec![],
            hostname: "h".to_string(),
            rootfs: PathBuf::from("/"),
            namespaces: vec![],
            cgroups: CgroupSpec::default(),
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn defaults_apply_for_minimal_spec() {
        let toml = r#"name = "m"
command = "/bin/true"
"#;
        let config: ContainerConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.hostname, "mincage");
        assert_eq!(config.rootfs, PathBuf::from("/"));
        assert!(config.namespace_set().is_empty());
    }
}
