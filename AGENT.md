# CRUST-OS agent guide

## Mission and current state

Help build an educational x86-64 operating system through researched, reviewable phases. Today this is a planning-stage repository: no kernel, boot path, build system, or tested toolchain is claimed. The proposed starting design is QEMU, bare-metal Rust with \`no_std\`, selective assembly, and a small monolithic kernel. Treat each as a revisable proposal, not a settled implementation.

## Non-negotiable repository habits

- Follow [AGENTS.md](AGENTS.md), preserve the existing license and unrelated files, and never generate .pnpm-store or unnecessary artifacts.
- During documentation-only tasks, create or update only authorized documentation and placeholders. Do not install tools, initialize Cargo, add source, or run builds.
- Do not change Git history or claim tests or boots that did not occur.
- Keep documentation navigable from [docs/README.md](docs/README.md); prefer one canonical explanation per subject.

## Architecture and Rust engineering

- Keep hardware assumptions explicit and isolate architecture-specific code behind small interfaces when implementation is later approved.
- Favor simple, inspectable kernel components. Add an abstraction only when a concrete use justifies it.
- Use ownership and lifetimes to express resource ownership. Treat raw pointers, MMIO, page tables, interrupt entry, and context switching as potential unsafe boundaries.
- For each future unsafe block, state its invariants, preconditions, and why a safe alternative is unavailable; review aliasing, alignment, lifetime, concurrency, and initialization.
- Define boot, memory-map, driver, and user/kernel contracts before relying on them. Never infer a hardware guarantee from QEMU behavior alone.

## Research and decisions

- Label facts, proposals, assumptions, and questions. Cite specifications or authoritative documentation and note relevant revision or platform assumptions.
- Record consequential choices in the [decision log](docs/09-project-management/decision-log.md), including options, rationale, consequences, and a revisit trigger.
- Keep unresolved decisions in [open questions](docs/06-research/open-questions.md). Do not quietly convert a proposal into a requirement.
- Recheck version-sensitive tool instructions and compatibility when the relevant phase begins.

## Phase workflow

Before implementing a future feature:

1. Read the [architecture](docs/03-system-architecture/architecture-overview.md), the relevant [phase plan](docs/README.md#development-phases), prerequisites, and open decisions.
2. Explain the proposed approach, expected files and dependencies, hardware assumptions, risks, and validation method.
3. Update the corresponding Markdown plan and decision record.
4. If the task is planning-only, stop before implementation. Begin implementation only when the user explicitly authorizes a specific phase and scope.
5. Stay within that authorization; report observed results, missing evidence, and remaining issues in the phase and [progress tracker](docs/09-project-management/progress-tracker.md).

For authorized future implementation, use the narrowest checks that demonstrate each acceptance criterion, record exact environment and outcomes, and distinguish emulator behavior from hardware validation. A feature is Completed only when its documented acceptance evidence is met.

## Reporting

Use the statuses Not Started, In Research, Planned, In Progress, Blocked, and Completed. Report what changed, why, what was actually checked, and what remains uncertain. Follow the [documentation standard](docs/08-engineering-standards/documentation-standards.md) and [review checklist](docs/08-engineering-standards/review-checklist.md).
