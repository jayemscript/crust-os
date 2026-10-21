# Documentation index

This repository documents a proposed operating system; it contains no runnable kernel. Begin with the [root overview](../README.md) and [roadmap](../ROADMAP.md). Status and decisions are tracked in [progress](09-project-management/progress-tracker.md) and the [decision log](09-project-management/decision-log.md).

## Project and learning

- [Vision](00-project-overview/vision.md), [objectives](00-project-overview/objectives.md), [scope](00-project-overview/scope.md), and [glossary](00-project-overview/glossary.md)
- [Rust fundamentals](01-learning-path/rust-fundamentals.md), [computer architecture](01-learning-path/computer-architecture.md), [OS theory](01-learning-path/operating-system-theory.md), [memory and pointers](01-learning-path/memory-and-pointers.md), and [learning resources](01-learning-path/learning-resources.md)
- [Prerequisites](02-development-environment/prerequisites.md), [toolchain](02-development-environment/toolchain-plan.md), [QEMU](02-development-environment/qemu-plan.md), and [debugging](02-development-environment/debugging-plan.md)

## Architecture

- [Overview](03-system-architecture/architecture-overview.md), [boot process](03-system-architecture/boot-process.md), [kernel design](03-system-architecture/kernel-design.md), [memory model](03-system-architecture/memory-model.md), [hardware abstraction](03-system-architecture/hardware-abstraction.md), and [security model](03-system-architecture/security-model.md)

## Development phases

| Foundation and kernel | Core services | Later capabilities |
| --- | --- | --- |
| [00 Foundation](04-development-phases/phase-00-foundation.md) | [04 Memory](04-development-phases/phase-04-memory-management.md) | [08 System calls](04-development-phases/phase-08-system-calls.md) |
| [01 Bootloader](04-development-phases/phase-01-bootloader.md) | [05 Processes](04-development-phases/phase-05-process-management.md) | [09 Shell](04-development-phases/phase-09-shell.md) |
| [02 Kernel](04-development-phases/phase-02-kernel.md) | [06 Drivers](04-development-phases/phase-06-device-drivers.md) | [10 Networking](04-development-phases/phase-10-networking.md) |
| [03 Interrupts](04-development-phases/phase-03-interrupts.md) | [07 File system](04-development-phases/phase-07-file-system.md) | [11 Advanced](04-development-phases/phase-11-advanced-features.md) |

## Plans, evidence, and standards

- Implementation plans: [kernel](05-implementation-plans/kernel-implementation.md), [memory](05-implementation-plans/memory-implementation.md), [scheduler](05-implementation-plans/scheduler-implementation.md), [drivers](05-implementation-plans/drivers-implementation.md), [file system](05-implementation-plans/filesystem-implementation.md), and [syscalls](05-implementation-plans/syscall-implementation.md)
- Research: [references](06-research/references.md), [Rust OS development](06-research/rust-os-development.md), [kernel comparison](06-research/os-kernel-comparison.md), [experiments](06-research/experimental-ideas.md), and [open questions](06-research/open-questions.md)
- Validation plans: [test strategy](07-testing-and-validation/test-strategy.md), [boot](07-testing-and-validation/boot-validation.md), [memory safety](07-testing-and-validation/memory-safety.md), and [debugging](07-testing-and-validation/debugging-strategy.md)
- Engineering: [coding](08-engineering-standards/coding-standards.md), [documentation](08-engineering-standards/documentation-standards.md), [Git workflow](08-engineering-standards/git-workflow.md), and [review checklist](08-engineering-standards/review-checklist.md)
- Management: [milestones](09-project-management/milestones.md), [backlog](09-project-management/task-backlog.md), [progress](09-project-management/progress-tracker.md), [decisions](09-project-management/decision-log.md), and [changelog](09-project-management/changelog.md)
