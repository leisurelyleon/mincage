# 3. Linux-only scope

- Status: Accepted
- Date: 2026-05

## Context

The isolation primitives mincage demonstrates — namespaces, cgroups,
`pivot_root` — are Linux kernel features with no portable equivalent.

## Decision

Scope the project to Linux explicitly. The `nix` dependency is declared only for
`target_os = "linux"`, and the documentation states the constraint plainly.

## Consequences

- The implementation can use Linux primitives directly and honestly.
- The project does not pretend to a portability it cannot deliver.
- Building on non-Linux targets is unsupported by design, not by accident.
