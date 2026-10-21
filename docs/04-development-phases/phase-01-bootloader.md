# Phase 01 — Boot integration

## Overview
Establish a reproducible loader-to-kernel handoff; a custom bootloader is optional.

## Goals and learning objectives
Explain firmware modes, image loading, CPU entry state, and ownership of loader data.

## Required concepts
UEFI versus BIOS, x86-64 modes, executable image layout, and calling conventions.

## Prerequisites
Phase 00 plans; resolve the [boot questions](../06-research/open-questions.md) before coding.

## Technical architecture
Select firmware and loader protocol, then define entry registers, stack, mode, mappings, memory map, and diagnostic channel in a reviewed contract.

## Proposed implementation steps
Compare boot approaches; record a decision; specify the handoff; then, only after authorization, create the smallest image that reaches a known entry and reports success or failure.

## Important Rust and OS concepts
\`no_std\` entry, binary format, ABI, unsafe access to boot data, and firmware-to-kernel ownership.

## Expected components and file structure
Future boot integration files, target/linker configuration, and a minimal kernel entry as justified by the chosen protocol; exact paths wait for the decision record.

## Hardware considerations
Pin QEMU machine, CPU, firmware, boot medium, memory size, and early output.

## Dependencies on previous phases
Phase 00 establishes scope and evidence rules; phase 02 consumes the handoff.

## Potential risks
Assuming undocumented loader state, using an invalid memory map, or hiding failures behind a blank screen.

## Testing strategy
Later: cold boot repeatedly in a pinned QEMU profile; inspect entry state and negative boot cases with debugger/serial evidence.

## Acceptance criteria
Controlled entry is observable and the handoff contract is documented with reproducible settings.

## Deliverables
Selected boot contract, minimal boot path, and recorded observations after approval.

## Research references
[UEFI specifications](https://uefi.org/specifications), [AMD64 manual](https://docs.amd.com/v/u/en-US/24593_3.44_APM_Vol2), and [QEMU docs](https://www.qemu.org/docs/master/system/index.html).

## Checklist of tasks
- [ ] Compare firmware and loader options.
- [ ] Decide and document handoff and QEMU profile.
- [ ] Implement only after phase authorization.
- [ ] Capture repeatable boot evidence.

## Current status
**Not Started.**
