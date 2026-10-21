# Architecture overview

**Status: proposal, not an implemented design.** The initial target is x86-64 in QEMU, with a small monolithic Rust kernel using \`no_std\`. [Boot](boot-process.md), [memory](memory-model.md), [hardware](hardware-abstraction.md), and [security](security-model.md) give the detailed boundaries. Changes to foundational assumptions require a [decision record](../09-project-management/decision-log.md).

## Proposed flow

Firmware starts a loader; the loader supplies an explicit handoff and transfers control to a kernel entry point. The kernel establishes diagnostics, exceptions, memory ownership, and only then more complex execution and devices. Later, isolated processes invoke a narrow system-call interface. This is a sequencing model, not a verified boot path.

## Boundaries to define before code

- Firmware/loader versus kernel: entry state, image layout, memory map, framebuffer or serial information, and ownership of boot data.
- Core versus architecture code: CPU mode, interrupts, paging, and context switch mechanics.
- Core versus devices: resource ownership, initialization, interrupt routing, and error reporting.
- Kernel versus user code: address-space permissions, syscall ABI, handles, and fault containment.

An existing loader may be used. A custom loader is not assumed. The initial platform should be kept narrow enough that each boundary can be inspected under QEMU.
