# Hardware abstraction

**Proposal:** support one documented QEMU x86-64 profile initially. Keep CPU operations, interrupt controller, clocks, console, and block devices behind narrow interfaces once there is a second caller or a meaningful test seam. Avoid pretending the first kernel is portable to arbitrary machines.

Each driver plan should name its discovery mechanism, I/O method (port I/O or MMIO), interrupts or polling, DMA assumptions, memory barriers, ownership, and error states. Early diagnostic output may be a deliberately minimal special case before the driver phase. The chosen console and storage devices remain open decisions.

No physical device support is implied by successful emulation. See [driver plan](../05-implementation-plans/drivers-implementation.md) and [QEMU plan](../02-development-environment/qemu-plan.md).
