# 1. Separating logic from syscalls

- Status: Accepted
- Date: 2026-05

## Context

Container runtimes are inherently privileged and platform-specific, which makes
them hard to test: the interesting logic is entangled with syscalls that need
root and a Linux kernel.

## Decision

Split the system in two. `mincage-core` holds all decisions as pure, validated
data with no syscalls. `mincage-runtime` enacts those decisions behind an
`IsolationBackend` trait, with a real `LinuxBackend` and a recording
`NoopBackend` for tests.

## Consequences

- The majority of the system is unit-tested without privileges.
- Lifecycle orchestration is verified by asserting the recorded call sequence.
- The privileged surface is small, explicit, and concentrated in one file.
