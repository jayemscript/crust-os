# Kernel organization comparison

This is a design comparison, not a claim that one model is universally better.

| Model | Learning and engineering tradeoff |
| --- | --- |
| Monolithic | Early components share an address space, simplifying first boot and calls; faults and unsafe code have broad reach |
| Microkernel | More services can live outside the kernel, but message passing and isolation must exist early |
| Hybrid | Boundary choices vary; the label alone does not specify mechanisms |

**Proposal:** start monolithic and modular so boot, memory, and exception work remain tractable. Revisit this choice if measured complexity or isolation goals justify moving services. Record any such change in the [decision log](../09-project-management/decision-log.md).
