# Phase 04 — Memory management

## Overview
Account for physical memory and build explicit virtual mappings.

## Goals and learning objectives
Explain frame ownership, page translation, permissions, and allocation invariants.

## Required concepts
Boot memory map, page tables, page faults, alignment, and Rust pointer validity.

## Prerequisites
Phase 01 memory handoff and phase 03 page-fault diagnostics.

## Technical architecture
Reserve kernel and loader-owned regions; manage usable frames; create mappings with explicit owner and permissions. Add a heap only when a later component needs it.

## Proposed implementation steps
Audit memory map; design allocator invariants; define mapping API and invalidation behavior; implement and validate one layer at a time after authorization.

## Important Rust and OS concepts
Physical versus virtual addresses, aliasing, initialization, unsafe page-table manipulation, and allocator failure.

## Expected components and file structure
Future memory-map parser, frame allocator, page-table interface, and optional heap module.

## Hardware considerations
Page size, CPU paging features, reserved firmware regions, and MMIO ranges must match the selected platform.

## Dependencies on previous phases
Boot handoff supplies regions; exceptions diagnose faults; later processes require isolation.

## Potential risks
Freeing reserved frames, stale translations, writable aliases, and invalid Rust references.

## Testing strategy
Later: bounded fixtures for map parsing, mapping/permission tests, exhaustion cases, and deliberate page faults in QEMU.

## Acceptance criteria
All frames are accounted for; selected mappings and faults behave as specified; unsafe invariants are reviewed.

## Deliverables
Documented physical and virtual memory interfaces with observed evidence after approval.

## Research references
[AMD64 paging specification](https://docs.amd.com/v/u/en-US/24593_3.44_APM_Vol2) and [Rust Reference](https://doc.rust-lang.org/reference/).

## Checklist of tasks
- [ ] Resolve memory-map and virtual-layout decisions.
- [ ] Specify allocator and mapping invariants.
- [ ] Implement only after phase authorization.
- [ ] Validate protection and exhaustion behavior.

## Current status
**Not Started.**
