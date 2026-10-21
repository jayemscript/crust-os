# QEMU plan

QEMU's [system emulator documentation](https://www.qemu.org/docs/master/system/index.html) is the primary starting reference. Phase 01 should select and record the machine type, CPU model, firmware, boot media, memory size, console path, and whether acceleration is enabled. Pinning the chosen profile matters because defaults can vary by version and host.

The first observable result should be a controlled boot signal. Later evidence should include exception reports, memory-fault behavior, and device I/O. Keep a minimal known-good virtual hardware profile before adding devices.

**Open decision:** legacy BIOS versus UEFI, and the loader protocol, remain unresolved. No QEMU invocation has been run or verified.
