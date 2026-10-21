# Glossary

| Term | Meaning in this project |
| --- | --- |
| ABI | Binary-level agreement about calls, data layout, registers, and return behavior |
| Boot handoff | Data and CPU state a loader provides when it transfers control to the kernel |
| Frame | Fixed-size physical-memory unit managed by a physical allocator |
| Page | Fixed-size virtual-memory unit translated through page tables |
| IDT | x86 interrupt descriptor table, used to route exceptions and interrupts |
| MMIO | Memory-mapped I/O, where device registers appear at addresses |
| \`no_std\` | Rust compilation without the standard-library runtime; core remains available |
| Privilege level | CPU-enforced execution authority, such as x86 ring 0 or ring 3 |
| Process | Planned isolated execution context with its own address-space contract |
| Task | Schedulable execution context; may initially run in kernel space |
| Unsafe invariant | Condition that must hold for an unsafe operation to be sound |

For detailed designs, use the [architecture pages](../03-system-architecture/architecture-overview.md).
