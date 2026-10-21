# Phase 07 — Storage and file system

## Overview
Turn reliable block access into a minimal, comprehensible file interface.

## Goals and learning objectives
Understand block addressing, metadata, names, consistency, and read errors.

## Required concepts
Phase 06 block I/O, buffer ownership, disk image format, and bounds checking.

## Prerequisites
Selected block device validated; memory allocation policy sufficient for buffers.

## Technical architecture
Begin with a documented read-only image or simple format. Separate block driver from file naming and parsing; defer writes until consistency rules are designed.

## Proposed implementation steps
Choose format and fixture; define parser and file operations; implement read path; evaluate writes separately after authorization.

## Important Rust and OS concepts
Parsing untrusted bytes, lifetime of borrowed buffers, error propagation, and crash consistency.

## Expected components and file structure
Future block abstraction, format parser, file handles, and known test image fixture.

## Hardware considerations
Sector size, device capacity, cache behavior, and emulated media attachment must be explicit.

## Dependencies on previous phases
Phase 04 memory and phase 06 block I/O; phase 09 may consume files.

## Potential risks
Out-of-bounds metadata, corrupted images, and assuming a successful read means durable storage.

## Testing strategy
Later: known image reads, missing files, malformed metadata, bounds cases, and repeatability under QEMU.

## Acceptance criteria
Known files are read with checked limits; malformed images return defined errors. Write support requires separate criteria.

## Deliverables
Read-only file abstraction and observed fixture results after approval.

## Research references
Chosen file-format specification and [QEMU disk documentation](https://www.qemu.org/docs/master/system/index.html), selected during this phase.

## Checklist of tasks
- [ ] Choose format and read-only scope.
- [ ] Define parsing and ownership invariants.
- [ ] Implement only after phase authorization.
- [ ] Verify valid and malformed images.

## Current status
**Not Started.**
