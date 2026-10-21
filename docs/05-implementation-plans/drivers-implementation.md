# Driver implementation plan

**Status: Planned documentation; implementation Not Started.** See [phase 06](../04-development-phases/phase-06-device-drivers.md).

Choose virtual devices based on the next required service, not broad hardware support. For each device, record QEMU model and version, discovery, I/O registers, interrupts, DMA if present, initialization order, ownership, and recoverable errors. Early diagnostic output from phase 02 can remain a special case until a general console interface is justified.

After authorization, introduce a minimal resource interface, implement one device, and validate absence and error paths before adding another. A block device is required for phase 07; an input/output path is required for the eventual shell. Review volatile versus atomic access and any shared buffer lifetime.
