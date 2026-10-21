# Memory and pointers

Memory work requires two simultaneous models: Rust's rules for references and aliasing, and the CPU's translation and protection of addresses. A valid mapped address is not automatically a valid Rust reference. Conversely, Rust ownership does not configure page permissions.

## Study checklist

- [ ] Explain virtual versus physical addresses and page-frame ownership.
- [ ] Explain alignment, initialization, lifetimes, and exclusive mutable access.
- [ ] Identify when raw pointers or volatile operations are necessary.
- [ ] State what a page fault can and cannot prove about pointer soundness.
- [ ] Review allocator and mapping invariants before phase 04.

See the proposed [memory model](../03-system-architecture/memory-model.md) and [memory safety validation plan](../07-testing-and-validation/memory-safety.md).
