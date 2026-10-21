# Phase 08 — System calls and user mode

## Overview
Define a narrow service boundary between isolated user code and the kernel.

## Goals and learning objectives
Understand privilege transitions, ABI stability, input validation, and fault containment.

## Required concepts
CPU privilege levels, process address spaces, interrupts/exceptions, and calling conventions.

## Prerequisites
Phase 03 fault handling, phase 04 protection, and phase 05 process model.

## Technical architecture
Choose a supported entry mechanism and specify register/stack contract, syscall numbers, return values, handle ownership, and user-buffer validation.

## Proposed implementation steps
Document ABI; implement transition and a tiny call set; reject invalid pointers and unknown calls; validate user-mode return after authorization.

## Important Rust and OS concepts
Unsafe user-memory access, address validation, lifetime separation, and capability/handle design.

## Expected components and file structure
Future syscall dispatch, architecture entry/return glue, user-memory helpers, and small user ABI definitions.

## Hardware considerations
CPU entry instructions, ring transitions, interrupt state, and page permissions must match selected x86-64 features.

## Dependencies on previous phases
Processes and isolation are required; shell depends on the syscall boundary.

## Potential risks
Kernel privilege leakage, invalid user pointers, ABI confusion, and unbounded kernel work.

## Testing strategy
Later: valid calls, unknown calls, invalid addresses, boundary buffers, and a crashing user process without kernel loss.

## Acceptance criteria
A user process receives documented results for allowed calls; malformed requests fail without corrupting kernel state.

## Deliverables
Reviewed ABI, minimal syscall path, and fault evidence after approval.

## Research references
[AMD64 system programming manual](https://docs.amd.com/v/u/en-US/24593_3.44_APM_Vol2) and selected ABI specification.

## Checklist of tasks
- [ ] Decide entry method and initial call set.
- [ ] Specify validation and error rules.
- [ ] Implement only after phase authorization.
- [ ] Demonstrate allowed and rejected calls.

## Current status
**Not Started.**
