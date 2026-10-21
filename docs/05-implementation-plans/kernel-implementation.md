# Kernel implementation plan

**Status: Planned documentation; implementation Not Started.** This plan refines [phase 02](../04-development-phases/phase-02-kernel.md) and does not authorize code.

## Contract to decide

Phase 01 must supply a documented CPU state, stack, binary image, and boot-data lifetime. The kernel entry should validate the minimum handoff needed for diagnostics. Define one early output path and a panic behavior that is visible in the pinned QEMU profile.

## Sequence after authorization

1. Establish an entry and explicit initialization stages.
2. Preserve or copy boot information according to its ownership contract.
3. Make startup and panic observations distinguishable.
4. Add exception support only when phase 03 begins.

Expected future components are an entry module, platform initialization, diagnostic output, and panic path. Avoid a heap, general driver registry, or user-space API in the first kernel.

## Evidence and risks

Reproduce a normal boot and deliberate panic with the selected emulator settings. Review assumptions about stack alignment, static initialization, and any unsafe boot-data access. A visible message alone does not prove memory safety.
