//! The real Linux isolation backend, built on `nix` syscalls.

use std::fs;
use std::path::Path;

use nix::mount::{mount, umount2, MntFlags, MsFlags};
use nix::sched::{unshare, CloneFlags};
use nix::unistd::{chdir, pivot_root, sethostname};

use mincage_core::{CgroupSpec, MountFlag, MountPlan, Namespace, NamespaceSet};

use crate::backend::IsolationBackend;
use crate::error::RuntimeResult;

/// Maps a namespace set onto kernel clone flags.
fn clone_flags(namespaces: &NamespaceSet) -> CloneFlags {
    let mut flags = CloneFlags::empty();
    for namespace in namespaces.iter() {
        flags |= match namespace {
            Namespace::Pid => CloneFlags::CLONE_NEWPID,
            Namespace::Mount => CloneFlags::CLONE_NEWNS,
            Namespace::Uts => CloneFlags::CLONE_NEWUTS,
            Namespace::Network => CloneFlags::CLONE_NEWNET,
            Namespace::User => CloneFlags::CLONE_NEWUSER,
            Namespace::Ipc => CloneFlags::CLONE_NEWIPC,
        };
    }
    flags
}

/// Maps core mount flags onto kernel mount flags.
fn to_ms_flags(flags: &[MountFlag]) -> MsFlags {
    let mut ms = MsFlags::empty();
    for flag in flags {
        ms |= match flag {
            MountFlag::Bind => MsFlags::MS_BIND,
            MountFlag::Rec => MsFlags::MS_REC,
            MountFlag::Private => MsFlags::MS_PRIVATE,
            MountFlag::NoSuid => MsFlags::MS_NOSUID,
            MountFlag::NoDev => MsFlags::MS_NODEV,
            MountFlag::NoExec => MsFlags::MS_NOEXEC,
            MountFlag::ReadOnly => MsFlags::MS_RDONLY,
        };
    }
    ms
}

/// The production backend performing real syscalls.
#[derive(Debug, Default)]
pub struct LinuxBackend;

impl IsolationBackend for LinuxBackend {
    fn unshare(&self, namespaces: &NamespaceSet) -> RuntimeResult<()> {
        unshare(clone_flags(namespaces))?;
        Ok(())
    }

    fn set_hostname(&self, name: &str) -> RuntimeResult<()> {
        sethostname(name)?;
        Ok(())
    }

    fn pivot_into(&self, rootfs: &Path) -> RuntimeResult<()> {
        // Make the whole mount tree private so changes don't leak to the host.
        mount(
            None::<&str>,
            "/",
            None::<&str>,
            MsFlags::MS_REC | MsFlags::MS_PRIVATE,
            None::<&str>,
        )?;

        // Bind the new root onto itself so it becomes a valid mount point.
        mount(
            Some(rootfs),
            rootfs,
            None::<&str>,
            MsFlags::MS_BIND | MsFlags::MS_REC,
            None::<&str>,
        )?;

        // Prepare a place to stash the old root, then pivot.
        let put_old = rootfs.join(".oldroot");
        fs::create_dir_all(&put_old)?;
        pivot_root(rootfs, &put_old)?;
        chdir("/")?;

        // Detach and remove the old root.
        umount2("/.oldroot", MntFlags::MNT_DETACH)?;
        let _ = fs::remove_dir("/.oldroot");

        Ok(())
    }

    fn apply_mounts(&self, plan: &MountPlan) -> RuntimeResult<()> {
        for op in plan.ops() {
            mount(
                op.source(),
                op.target(),
                op.fstype(),
                to_ms_flags(op.flags()),
                None::<&str>,
            )?;
        }
        Ok(())
    }

    fn apply_cgroups(&self, name: &str, spec: &CgroupSpec, pid: i32) -> RuntimeResult<()> {
        if spec.is_empty() {
            return Ok(());
        }

        let base = Path::new("/sys/fs/cgroup").join("mincage").join(name);

        // Graceful degradation: if cgroup management is unavailable in this
        // environment, log it and continue without resource limits.
        if let Err(err) = fs::create_dir_all(&base) {
            eprintln!("warning: cgroups unavailable ({err}); continuing without limits.");
            return Ok(());
        }

        for (file, contents) in spec.controller_files() {
            if let Err(err) = fs::write(base.join(&file), &contents) {
                eprintln!("warning: could not set {file} ({err}); skipping this limit.");
            }
        }

        if let Err(err) = fs::write(base.join("cgroup.procs"), pid.to_string()) {
            eprintln!("warning: could not attach pid {pid} to cgroup ({err}).");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::clone_flags;
    use mincage_core::{Namespace, NamespaceSet};
    use nix::sched::CloneFlags;

    #[test]
    fn maps_namespaces_to_clone_flags() {
        let set = NamespaceSet::new().with(Namespace::Pid).with(Namespace::Uts);
        let flags = clone_flags(&set);
        assert!(flags.contains(CloneFlags::CLONE_NEWPID));
        assert!(flags.contains(CloneFlags::CLONE_NEWUTS));
        assert!(!flags.contains(CloneFlags::CLONE_NEWNET));
    }

    #[test]
    fn empty_set_maps_to_no_flags() {
        assert_eq!(clone_flags(&NamespaceSet::new()), CloneFlags::empty());
    }
}
