# Phase 06 — Device drivers

## Overview
Build only the emulated device support needed by the next milestone.

## Goals and learning objectives
Understand device discovery, register access, interrupts, polling, and error handling.

## Required concepts
Port I/O, MMIO, memory ordering, IRQ ownership, and device-specific protocols.

## Prerequisites
Phase 02 diagnostics, phase 03 interrupt path, and phase 04 mapping rules.

## Technical architecture
Use narrow interfaces for console and selected storage or input devices. Keep a driver responsible for its resources and initialization state.

## Proposed implementation steps
Choose the QEMU device model; document registers and lifecycle; implement one device path and its failure cases after authorization.

## Important Rust and OS concepts
Volatile access, unsafe MMIO invariants, DMA buffer ownership, and synchronization with handlers.

## Expected components and file structure
Future device interface and separate modules for only the selected virtual devices.

## Hardware considerations
Emulator model and version, I/O base, IRQ routing, and DMA constraints must be recorded.

## Dependencies on previous phases
Interrupts and memory underpin drivers; phase 07 needs block I/O.

## Potential risks
Wrong device assumptions, lost interrupts, unsound buffer sharing, and silent I/O errors.

## Testing strategy
Later: probe known devices, exercise normal transfers and absence/failure paths, and record QEMU configuration.

## Acceptance criteria
The selected device can be initialized and used repeatedly, with bounded errors and documented assumptions.

## Deliverables
Minimal device interfaces and selected driver evidence after approval.

## Research references
[QEMU device documentation](https://www.qemu.org/docs/master/system/index.html) plus the chosen device's authoritative specification.

## Checklist of tasks
- [ ] Select and document the minimal device set.
- [ ] Define resource and error contracts.
- [ ] Implement only after phase authorization.
- [ ] Demonstrate normal and failure paths.

## Current status
**Not Started.**
