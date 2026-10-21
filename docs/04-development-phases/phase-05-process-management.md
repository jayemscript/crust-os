# Phase 05 — Tasks and processes

## Overview
Introduce schedulable execution, then isolated processes when memory protection is ready.

## Goals and learning objectives
Understand saved CPU context, stack ownership, scheduling policy, and address-space switches.

## Required concepts
Interrupt return, timer ticks, virtual memory, privilege levels, and concurrency.

## Prerequisites
Reliable phase 03 timer/exception paths and phase 04 memory interfaces.

## Technical architecture
Begin with a small task-state model and simple policy. Separate mechanism from policy; add user process isolation only after a privilege boundary is specified.

## Proposed implementation steps
Specify context and lifecycle; demonstrate cooperative switching; consider preemption; define process address spaces and failure behavior after authorization.

## Important Rust and OS concepts
Pinned stacks, aliasing across switches, atomics, critical sections, and ownership of task resources.

## Expected components and file structure
Future task state, context-switch architecture glue, scheduler, and process/address-space manager.

## Hardware considerations
Saved register set, interrupt stack, timer source, and CPU privilege transition must match x86-64 behavior.

## Dependencies on previous phases
Phase 03 supplies timing/fault paths; phase 04 supplies stacks and mappings; phase 08 later adds syscalls.

## Potential risks
Corrupted context, use-after-free stacks, starvation, and leaking kernel mappings to user mode.

## Testing strategy
Later: multiple tasks with observable progress, task termination, timer stress, and fault attribution.

## Acceptance criteria
Tasks switch without state corruption; process isolation claims are supported by fault tests before user programs rely on them.

## Deliverables
Task/scheduler design and verified minimal execution path after approval.

## Research references
[AMD64 system programming manual](https://docs.amd.com/v/u/en-US/24593_3.44_APM_Vol2).

## Checklist of tasks
- [ ] Decide initial scheduling policy and process boundary.
- [ ] Document context and stack invariants.
- [ ] Implement only after phase authorization.
- [ ] Observe progress and isolation failures.

## Current status
**Not Started.**
