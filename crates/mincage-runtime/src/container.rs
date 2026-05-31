//! Container lifecycle orchestration.
//!
//! `Container<B>` is generic over a backend, so the child-preparation sequence
//! is unit-testable with `NoopBackend`. `launch` performs the real fork/exec
//! dance with `LinuxBackend`.

use std::ffi::CString;

use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::{ForkResult, execvp, fork};

use mincage_core::{ContainerConfig, MountPlan};

use crate::backend::IsolationBackend;
use crate::error::RuntimeResult;
use crate::linux_backend::LinuxBackend;

/// Orchestrates the privileged steps of a container launch via a backend.
pub struct Container<B: IsolationBackend> {
    backend: B,
}

impl<B: IsolationBackend> Container<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    /// Read access to the backend (used in tests to inspect recorded calls).
    pub fn backend(&self) -> &B {
        &self.backend
    }

    /// Enters the configured namespaces (performed by the parent before fork).
    pub fn enter_namespaces(&self, config: &ContainerConfig) -> RuntimeResult<()> {
        self.backend.unshare(&config.namespace_set())
    }

    /// The steps performed inside the child before exec: set the hostname,
    /// pivot into the new root, and establish the standard mounts.
    pub fn prepare_child(&self, config: &ContainerConfig) -> RuntimeResult<()> {
        self.backend.set_hostname(&config.hostname)?;
        self.backend.pivot_into(&config.rootfs)?;
        self.backend.apply_mounts(&MountPlan::standard())?;
        Ok(())
    }

    /// Applies cgroup limits to the child (performed by the parent post-fork).
    pub fn apply_cgroups(&self, config: &ContainerConfig, pid: i32) -> RuntimeResult<()> {
        self.backend
            .apply_cgroups(&config.name, &config.cgroups, pid)
    }
}

/// Replaces the current process image with the configured command.
fn exec_command(config: &ContainerConfig) -> RuntimeResult<()> {
    let command = CString::new(config.command.as_str())?;
    let mut argv: Vec<CString> = Vec::with_capacity(config.args.len() + 1);
    argv.push(command.clone());
    for arg in &config.args {
        argv.push(CString::new(arg.as_str())?);
    }
    execvp(command.as_c_str(), &argv)?;
    Ok(())
}

/// Launches a container: enters namespaces, forks, isolates the child, and
/// executes the command. Returns the child's exit code.
///
/// This performs real privileged syscalls and is exercised via the CLI rather
/// than unit tests; the testable logic lives in `Container::prepare_child`.
pub fn launch(config: &ContainerConfig) -> RuntimeResult<i32> {
    config.validate()?;

    let container = Container::new(LinuxBackend);
    container.enter_namespaces(config)?;

    match unsafe { fork() }? {
        ForkResult::Parent { child } => {
            container.apply_cgroups(config, child.as_raw())?;
            match waitpid(child, None)? {
                WaitStatus::Exited(_, code) => Ok(code),
                WaitStatus::Signaled(_, signal, _) => Ok(128 + signal as i32),
                _ => Ok(-1),
            }
        }
        ForkResult::Child => {
            if let Err(err) = container.prepare_child(config) {
                eprintln!("mincage: child setup failed: {err}");
                std::process::exit(126);
            }
            if let Err(err) = exec_command(config) {
                eprintln!("mincage: exec failed: {err}");
                std::process::exit(127);
            }
            unreachable!("execvp replaces the process image on success");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::NoopBackend;
    use std::path::PathBuf;

    fn sample_config() -> ContainerConfig {
        ContainerConfig {
            name: "test".to_string(),
            command: "/bin/true".to_string(),
            args: vec![],
            hostname: "host".to_string(),
            rootfs: PathBuf::from("/tmp/rootfs"),
            namespaces: vec![],
            cgroups: mincage_core::CgroupSpec::default(),
        }
    }

    #[test]
    fn prepare_child_runs_steps_in_order() {
        let container = Container::new(NoopBackend::new());
        container.prepare_child(&sample_config()).unwrap();

        assert_eq!(
            container.backend().calls(),
            vec![
                "set_hostname:host".to_string(),
                "pivot_into:/tmp/rootfs".to_string(),
                "apply_mounts:1".to_string(),
            ]
        );
    }

    #[test]
    fn enter_namespaces_invokes_unshare() {
        let container = Container::new(NoopBackend::new());
        container.enter_namespaces(&sample_config()).unwrap();
        assert_eq!(container.backend().calls(), vec!["unshare".to_string()]);
    }

    #[test]
    fn apply_cgroups_invokes_backend() {
        let container = Container::new(NoopBackend::new());
        container.apply_cgroups(&sample_config(), 1234).unwrap();
        assert_eq!(
            container.backend().calls(),
            vec!["apply_cgroups".to_string()]
        );
    }
}
