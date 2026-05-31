# 2. cgroups v2 with graceful degradation

- Status: Accepted
- Date: 2026-05

## Context

cgroup management is frequently restricted in nested or managed environments
(CI runners, cloud dev containers). A runtime that hard-fails when it cannot
write cgroup files is unusable in exactly the places people first try it.

## Decision

Target cgroups v2 (the unified hierarchy). When creating or writing the cgroup
fails, log the specific limitation and continue without resource limits rather
than aborting the launch. Process and filesystem isolation are unaffected.

## Consequences

- mincage runs in restricted environments, just without enforced limits there.
- The degradation is visible (logged), never silent.
- Where cgroups are available, limits are applied normally.
