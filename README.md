# mincage
A minimal container runtime in Rust demonstrating Linux process isolation from first principles. Uses namespaces, cgroups, and pivot_root to launch sandboxed processes with isolated PID, mount, and network views — built to show how containers actually work beneath Docker.
