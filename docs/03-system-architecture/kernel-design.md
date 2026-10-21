# Kernel design

**Proposal:** begin with a monolithic kernel because a single address space keeps early boot, exception, and memory experiments inspectable. This does not imply one unstructured module. Separate architecture-dependent CPU support, initialization, memory, tasks, devices, storage, and syscall boundary by clear ownership and interfaces.

Initialization should be staged: establish reliable diagnostics, validate the boot handoff, install exception handling, account for memory, then activate interrupts and later services. Each stage should state what is safe to call and what failure can be reported. Avoid an allocator or asynchronous execution until their invariants are documented.

The first kernel should expose only the capabilities needed for the next milestone. A module split or API is justified by a concrete caller, not a speculative future feature. See the [kernel implementation plan](../05-implementation-plans/kernel-implementation.md).
