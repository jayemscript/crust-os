# Phase 02 — Minimal kernel

## Overview
Turn the phase 01 entry into a small kernel with visible initialization and failure behavior.

## Goals and learning objectives
Understand \`no_std\` startup, panic handling, initialization order, and simple kernel boundaries.

## Required concepts
Rust core library, ABI, static data, volatile I/O, and boot-handoff lifetime.

## Prerequisites
Approved and validated phase 01 handoff and early output decision.

## Technical architecture
A staged entry validates boot data, initializes diagnostics, and stops predictably on errors. No heap or general driver framework is assumed.

## Proposed implementation steps
Specify entry and panic contracts; define initialization states; add minimal diagnostics; demonstrate successful and failed paths after authorization.

## Important Rust and OS concepts
Panic strategy, ownership of boot information, global state, unsafe boundaries, and error reporting without std.

## Expected components and file structure
Future kernel entry, architecture-specific initialization, diagnostics, and panic module; exact source layout follows the selected boot integration.

## Hardware considerations
Choose one QEMU-visible output path; do not generalize it as hardware-portable.

## Dependencies on previous phases
Phase 01 supplies CPU state and boot data; phase 03 will build on kernel diagnostics.

## Potential risks
Silent panic, undefined initialization order, or invalid references into loader-owned memory.

## Testing strategy
Later: observe cold boots, deliberate panic, invalid boot-data response, and symbolized debugger traces.

## Acceptance criteria
Initialization and failure are both observable and the kernel's entry assumptions are documented.

## Deliverables
Minimal kernel core and recorded QEMU evidence after approval.

## Research references
[Rust \`no_std\` guide](https://doc.rust-lang.org/embedded-book/intro/no-std.html) and [AMD64 manual](https://docs.amd.com/v/u/en-US/24593_3.44_APM_Vol2).

## Checklist of tasks
- [ ] Review handoff lifetime and entry ABI.
- [ ] Specify panic and diagnostic behavior.
- [ ] Implement only after phase authorization.
- [ ] Observe normal and fault paths.

## Current status
**Not Started.**
