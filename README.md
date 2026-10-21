# crust-os

**Status: planning-stage educational project. No bootable operating system or kernel has been implemented.**

crust-os is a long-term exercise in learning how an operating system reaches the screen, manages memory and execution, and provides services to programs. The first proposed target is an x86-64 machine in QEMU. The initial design proposes a small monolithic kernel written primarily in bare-metal Rust with \`no_std\`; assembly would be used only where the hardware interface requires it. These choices remain subject to the [decision process](docs/09-project-management/decision-log.md).

## Why build it?

The project favors understanding over feature count. Each milestone should explain its hardware assumptions, make a narrow change, and define observable evidence before the next milestone begins. It is intended for a developer learning Rust systems programming, x86-64 architecture, and OS design while building a coherent system.

## Intended capabilities

The [roadmap](ROADMAP.md) progresses from boot handoff and minimal kernel diagnostics through exceptions, memory management, scheduling, processes, drivers, storage, system calls, and a basic shell. Networking and more advanced features are future work. None of these capabilities is presently available.

## Architecture proposal

- **Platform:** x86-64 under QEMU first; real hardware is outside the initial scope.
- **Boot:** compare UEFI and legacy BIOS and evaluate existing bootloader protocols before choosing a handoff.
- **Kernel:** a deliberately small monolithic core with explicit module boundaries.
- **Memory and safety:** explicit physical and virtual memory ownership; isolate and justify unavoidable unsafe operations.
- **User boundary:** defer user mode and system calls until memory isolation, exceptions, and task execution are understood.

See the [architecture overview](docs/03-system-architecture/architecture-overview.md) and [open questions](docs/06-research/open-questions.md).

## Development approach

This repository currently contains plans and research, not a buildable software project. Implementation of a phase requires a separate, explicit instruction. No setup, installation, boot, build, or test instructions here have been verified. [AGENT.md](AGENT.md) defines the intended workflow for later development.

## Start reading

1. [Project overview](docs/00-project-overview/vision.md) and [scope](docs/00-project-overview/scope.md)
2. [Documentation index](docs/README.md) and [learning path](docs/01-learning-path/rust-fundamentals.md)
3. [Roadmap](ROADMAP.md) and [phase plans](docs/04-development-phases/phase-00-foundation.md)
4. [Research references](docs/06-research/references.md) and [decision log](docs/09-project-management/decision-log.md)

The [research directory](research/README.md) describes how to add evidence without overstating conclusions. The existing [LICENSE](LICENSE) remains in place.
