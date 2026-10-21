# Phase 09 — Basic shell

## Overview
Provide a small interactive user program to exercise process, I/O, and file services.

## Goals and learning objectives
Understand command parsing, user-space error handling, and the OS service surface.

## Required concepts
Phase 08 syscall ABI, console input/output, and phase 07 file-service semantics.

## Prerequisites
Stable process launch and service calls, phase 07 read-only file access, and a documented console path.

## Technical architecture
A single-process text prompt with a tiny documented command set is sufficient. Commands should call existing services rather than reach into kernel internals.

## Proposed implementation steps
Choose commands and line limits; define error messages; implement prompt and parsing; exercise service calls after authorization.

## Important Rust and OS concepts
Bounded buffers, UTF-8 assumptions, user-space ownership, and separation of policy from kernel mechanism.

## Expected components and file structure
Future user program, command parser, and any minimal user ABI wrapper justified by phase 08.

## Hardware considerations
Input and output are through selected virtual console devices; no display or keyboard standard is presumed.

## Dependencies on previous phases
Processes, syscalls, console I/O, and the phase 07 file service.

## Potential risks
Unbounded input, misleading command results, and accidental kernel-specific coupling.

## Testing strategy
Later: prompt, empty line, valid commands, unknown command, long input, and I/O failure.

## Acceptance criteria
The prompt repeatedly accepts bounded input and reports documented results without destabilizing the kernel.

## Deliverables
Minimal user shell and interaction transcript after approval.

## Research references
Project [syscall design](../05-implementation-plans/syscall-implementation.md) and selected console documentation.

## Checklist of tasks
- [ ] Define command set and input bounds.
- [ ] Confirm required syscalls exist.
- [ ] Implement only after phase authorization.
- [ ] Observe valid and error interactions.

## Current status
**Not Started.**
