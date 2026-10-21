# Memory model

**Fact:** x86-64 page translation and protection are controlled by processor-defined structures; consult the [AMD64 manual](https://docs.amd.com/v/u/en-US/24593_3.44_APM_Vol2) for the exact mode and features chosen. **Proposal:** make physical-frame ownership and virtual mappings separate concepts.

The boot handoff must identify usable, reserved, and firmware-owned regions. The kernel should reserve its image, stack, boot data, MMIO regions, and page tables before handing frames to an allocator. Mapping code should make page size, permissions, address-space owner, and lifetime explicit. An allocator for heap objects is deferred until a consumer requires it.

**Safety rule:** a mapped address is not by itself a valid Rust reference. Initialization, alignment, aliasing, and lifetime invariants still apply. Page faults help diagnose bad access but do not prove Rust soundness. See [memory safety](../07-testing-and-validation/memory-safety.md).

**Open decisions:** boot memory-map format and lifetime, page-table ownership, initial virtual layout, frame allocator policy, and whether an early heap is necessary.
