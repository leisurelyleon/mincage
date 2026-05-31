# Architecture

`mincage` is a Rust workspace split so that all container *logic* is pure and
testable, while privileged *syscalls* are isolated behind a single seam.

## Crates

```text
mincage-core + pure data + validation: spec, namespaces, cgroup limits, mount plans (no syscalls) mincage-runtime   privileged layer: IsolationBackend trait, LinuxBackend syscalls, lifecycle
mincage-cli the `mincage` binary: run / spec

Dependencies flow inward toward `mincage-core`, which depends on nothing
platform-specific.

## The logic / syscall seam

The central design choice is the `IsolationBackend` trait. All decisions about
*what* to isolate (which namespaces, which limits, which mounts) are pure data
produced and validated in `mincage-core`. The trait defines *how* those
decisions are enacted:

- `LinuxBackend` performs the real `nix` syscalls (`unshare`, `sethostname`,
  `pivot_root`, `mount`, cgroup writes).
- `NoopBackend` records the calls it would make, enabling the lifecycle
  orchestration to be unit-tested with zero privileges.

This is why the majority of the system has tests that pass anywhere, while the
privileged code remains real rather than mocked away.

## Launch sequence

1. Parent enters the configured namespaces (`unshare`).
2. Parent forks.
3. Child sets its hostname, pivots into the new root filesystem, and applies
   the standard mounts (a fresh `/proc`).
4. Parent applies cgroup limits to the child and waits for it to exit.
5. Child execs the configured command, replacing its process image.
