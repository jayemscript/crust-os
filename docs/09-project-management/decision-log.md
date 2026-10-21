# Architecture decision log

No boot, loader, target, device, memory-layout, or syscall design has been approved. The project context proposes x86-64, QEMU first, a small monolithic kernel, and Rust \`no_std\`; these are **proposals** to review as phases begin.

## Decision record template

### ADR-NNN — Short decision title

- **Status:** Proposed / Accepted / Superseded
- **Date:** Record the actual decision date; do not invent one.
- **Context and evidence:** What problem and sources prompted the choice?
- **Options considered:** Include plausible alternatives and their costs.
- **Decision and rationale:** State the chosen contract and why.
- **Consequences:** New dependencies, constraints, and risks.
- **Validation:** What observation would challenge or confirm the choice?
- **Revisit trigger:** When should it be reconsidered?
- **Related phase:** Link the owning phase and affected documents.

## Pending decisions

The [open-question register](../06-research/open-questions.md) owns unresolved items. Do not create an Accepted record merely because a proposal appears in this repository.
