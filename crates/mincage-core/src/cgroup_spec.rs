//! Resource limits expressed as pure data, rendered into cgroup v2 file writes.

use serde::{Deserialize, Serialize};

/// A cgroup v2 resource specification. All limits are optional; an empty spec
/// applies no limits.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CgroupSpec {
    /// Maximum memory in bytes (`memory.max`).
    pub memory_max_bytes: Option<u64>,
    /// CPU quota in microseconds per period (`cpu.max` numerator).
    pub cpu_quota_us: Option<u64>,
    /// CPU period in microseconds (`cpu.max` denominator); defaults to 100000.
    pub cpu_period_us: Option<u64>,
}

impl CgroupSpec {
    /// True when no limits are configured.
    pub fn is_empty(&self) -> bool {
        self.memory_max_bytes.is_none() && self.cpu_quota_us.is_none()
    }

    /// Renders the limits as `(filename, contents)` pairs for cgroup v2.
    pub fn controller_files(&self) -> Vec<(String, String)> {
        let mut files = Vec::new();

        if let Some(memory) = self.memory_max_bytes {
            files.push(("memory.max".to_string(), memory.to_string()));
        }

        if let Some(quota) = self.cpu_quota_us {
            let period = self.cpu_period_us.unwrap_or(100_000);
            files.push(("cpu.max".to_string(), format!("{quota} {period}")));
        }

        files
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_spec_has_no_files() {
        assert!(CgroupSpec::default().is_empty());
        assert!(CgroupSpec::default().controller_files().is_empty());
    }

    #[test]
    fn renders_memory_and_cpu() {
        let spec = CgroupSpec {
            memory_max_bytes: Some(104_857_600),
            cpu_quota_us: Some(50_000),
            cpu_period_us: Some(100_000),
        };
        let files = spec.controller_files();
        assert!(files.contains(&("memory.max".to_string(), "104857600".to_string())));
        assert!(files.contains(&("cpu.max".to_string(), "50000 100000".to_string())));
    }

    #[test]
    fn cpu_period_defaults_when_absent() {
        let spec = CgroupSpec {
            memory_max_bytes: None,
            cpu_quota_us: Some(25_000),
            cpu_period_us: None,
        };
        let files = spec.controller_files();
        assert_eq!(files, vec![("cpu.max".to_string(), "25000 100000".to_string())]);
    }
}
