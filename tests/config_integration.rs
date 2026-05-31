//! End-to-end tests of spec loading and validation against on-disk fixtures.
//! These exercise the pure pipeline (parse -> validate -> derive) with no
//! privileges required.

use std::path::PathBuf;

use mincage_core::{ContainerConfig, Namespace};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn loads_and_validates_sample_spec() {
    let text = std::fs::read_to_string(fixture("sample-spec.toml")).unwrap();
    let config: ContainerConfig = toml::from_str(&text).unwrap();

    config.validate().expect("sample spec should be valid");
    assert_eq!(config.name, "demo");
    assert_eq!(config.hostname, "mincage-demo");
}

#[test]
fn sample_spec_isolates_expected_namespaces() {
    let text = std::fs::read_to_string(fixture("sample-spec.toml")).unwrap();
    let config: ContainerConfig = toml::from_str(&text).unwrap();
    let set = config.namespace_set();

    assert!(set.contains(Namespace::Pid));
    assert!(set.contains(Namespace::Mount));
    assert!(set.contains(Namespace::Uts));
    assert!(!set.contains(Namespace::Network));
}

#[test]
fn sample_spec_defines_resource_limits() {
    let text = std::fs::read_to_string(fixture("sample-spec.toml")).unwrap();
    let config: ContainerConfig = toml::from_str(&text).unwrap();

    assert_eq!(config.cgroups.memory_max_bytes, Some(134_217_728));
    assert!(!config.cgroups.is_empty());
    assert_eq!(config.cgroups.controller_files().len(), 2);
}
