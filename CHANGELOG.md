# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial workspace scaffold: mincage-core, mincage-runtime, mincage-cli.

## [0.1.0] - TBD

### Added
- Pure container specification model: namespaces, cgroup limits, mount plans.
- Linux isolation backend using namespaces, cgroups v2, and pivot_root.
- Graceful degradation when cgroup controls are unavailable.
- CLI to launch a sandboxed process from a spec.

[Unreleased]: https://github.com/leisurelyleon/mincage/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/leisurelyleon/mincage/releases/tag/v0.1.0
