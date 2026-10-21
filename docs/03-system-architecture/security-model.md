# Security model

**Current state:** no security mechanism has been implemented. This is an educational OS, not a hardened platform. Planned boundaries still matter because they constrain later design.

**Proposal:** keep kernel memory inaccessible to user code, make user mappings explicit, validate syscall arguments and buffer ranges, and prevent a user fault from silently corrupting kernel state. Before user mode exists, isolate unsafe hardware operations through documented invariants and reduce global mutable state.

The first threat model is accidental faults and malformed calls from a local user program under QEMU. Hostile devices, side channels, secure boot, cryptographic trust, and production exploit resistance are deferred. A later phase must revisit these boundaries if its threat model expands.

See [syscall plan](../05-implementation-plans/syscall-implementation.md) and [memory-safety plan](../07-testing-and-validation/memory-safety.md).
