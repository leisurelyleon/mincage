# Running mincage

## Build

```bash
cargo build --release
```

## Spec file

Generate a starting spec:

```bash
./target/release/mincage spec > spec.toml
```

## Launch

```bash
./target/release/mincage run --spec spec.toml
```

## Privileges

Creating namespaces and managing cgroups requires privileges. Depending on your
environment you may need `sudo`:

```bash
sudo ./target/release/mincage run --spec spec.toml
```

### Environment notes

- **Linux only.** mincage uses Linux-specific syscalls.
- **Containers within containers.** When run inside an already-containerized
  environment (such as a CI runner or a cloud development container), some
  operations may be restricted by the host. In particular, cgroup management may
  be unavailable. When that happens, mincage logs the limitation and continues
  *without* resource limits rather than failing — process and filesystem
  isolation still apply.
- **User namespaces.** Running without root typically requires enabling the
  user namespace (add `"user"` to the spec's `namespaces`), which remaps the
  caller to root *inside* the container only.
