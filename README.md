# mincage

> A minimal container runtime demonstrating Linux process isolation from first principles.

`mincage` launches a process in an isolated sandbox using the same Linux kernel
primitives that power Docker and containerd: namespaces (PID, mount, UTS,
network), cgroups v2 for resource limits, and `pivot_root` for an isolated root
filesystem. It is built to show how containers actually work beneath the tools
that hide these mechanics — not to wrap an existing runtime.

## The Problem

"Containers" are often treated as magic. In reality they are ordinary processes
that the kernel has given isolated *views* of the system. `mincage` makes that
concrete by assembling those views directly from syscalls, with the isolation
*logic* kept pure and testable and the privileged *syscalls* isolated behind a
clear boundary.

## Architecture

```
mincage-core      pure logic: container spec, namespace set, cgroup + mount plans (no syscalls)
mincage-runtime   the privileged layer: real namespace/cgroup/pivot_root syscalls via `nix`
mincage-cli       the binary: launch a sandboxed process from a spec
```

The split between `core` (pure, fully unit-tested) and `runtime` (privileged
syscalls behind a trait) is deliberate: it keeps the bulk of the system testable
without special privileges.

## Platform & Privileges

> **Linux only.** `mincage` uses Linux-specific syscalls and does not build on
> macOS or Windows.

Launching a real sandbox requires privileges to create namespaces and manage
cgroups. Depending on the environment, you may need to run the runtime with
`sudo`, and some environments restrict cgroup controls. When cgroup management
is unavailable, `mincage` logs the limitation and continues without resource
limits rather than failing. See [`docs/running.md`](docs/running.md).

## Build & Test

```bash
cargo build
cargo test          # core logic tests require no privileges
```

## Run

```bash
# May require sudo depending on your environment's namespace permissions.
sudo ./target/debug/mincage run --spec tests/fixtures/sample-spec.toml
```

## License

MIT — see [LICENSE](LICENSE).
