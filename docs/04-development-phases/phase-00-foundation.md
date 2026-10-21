# Phase 00 — Documentation foundation

## Overview
Define a credible, navigable plan before adding any executable component.

## Goals and learning objectives
Learn to separate source-backed facts from proposals and to express milestones as observable outcomes.

## Required concepts
Documentation structure, x86-64 boot vocabulary, \`no_std\` constraints, and dependency planning.

## Prerequisites
None beyond the repository and its existing license.

## Technical architecture
The [architecture overview](../03-system-architecture/architecture-overview.md) records a proposed x86-64, QEMU-first, monolithic \`no_std\` direction; boot and hardware contracts remain open.

## Proposed implementation steps
Organize root guidance, learning path, architecture, phase plans, research, validation, and governance as Markdown. Keep Makefile as a help-only placeholder.

## Important Rust and OS concepts
Rust ownership, CPU privilege, virtual memory, boot handoff, and unsafe invariants are documented as learning topics, not implemented mechanisms.

## Expected components and file structure
Root README, roadmap, agent guides, docs index and topic directories, research index, .gitignore, and documentation-only Makefile. No source directories are required.

## Hardware considerations
Record initial x86-64 and QEMU assumptions without selecting firmware or a device profile.

## Dependencies on previous phases
None.

## Potential risks
Overstating progress, broken navigation, duplicated explanations, and premature implementation decisions.

## Testing strategy
Review files, links, status wording, and scope directly; no commands, scripts, builds, or tests are part of this phase.

## Acceptance criteria
All requested documents exist, navigation is consistent, unimplemented work is labeled, and no Cargo or source files are added.

## Deliverables
Documentation foundation and explicit phase gates.

## Research references
See the [primary reference register](../06-research/references.md).

## Checklist of tasks
- [x] Define the documentation structure and architecture proposals.
- [x] Record phase dependencies, validation intentions, and open decisions.
- [x] Provide contributor and future-agent guidance.

## Current status
**Completed — documentation milestone only.** This status does not apply to any OS implementation phase.
