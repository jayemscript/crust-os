# Development roadmap

**Status: planning only.** Every implementation milestone below is Not Started. The order expresses technical dependencies, not promised dates. [Phase documents](docs/README.md#development-phases) provide the detailed checklists and acceptance plans.

| Milestone | Prerequisites | Objective and proposed deliverable | Acceptance evidence to seek later |
| --- | --- | --- | --- |
| 00 — Foundation | None | Establish scope, research method, architecture proposals, and phase gates | Documentation is navigable; decisions and unknowns are explicit |
| 01 — Boot integration | 00; x86-64 boot concepts | Select a firmware and loader handoff, then produce a minimal boot path | QEMU reaches a controlled entry point and reports a reproducible boot signal |
| 02 — Kernel core | 01; Rust \`no_std\` and ABI concepts | Define entry, panic behavior, and minimal diagnostics | Kernel starts consistently, reports initialization, and handles panic visibly |
| 03 — Exceptions and interrupts | 02; descriptor tables | Handle CPU exceptions and a chosen timer or device interrupt | Deliberate exceptions are identified; interrupt entry and return are repeatable |
| 04 — Memory management | 01 memory map; 03 fault handling | Track physical frames, establish virtual mappings, then add allocation only when needed | Mapping, protection, and fault cases are observed and accounted for |
| 05 — Tasks and processes | 03 timer; 04 isolation primitives | Introduce task state, context switching, scheduling, and eventually separate address spaces | Multiple tasks make progress; faults stay attributable to the right task |
| 06 — Device drivers | 02 diagnostics; 03 interrupts; 04 memory | Define a minimal device interface and implement only devices needed by later milestones | A selected emulated device can be initialized, exercised, and fail clearly |
| 07 — Storage and file system | 06 block I/O; 04 memory | Read persistent data and define a simple file abstraction | A known test image can be read with checked boundaries and errors |
| 08 — System calls and user mode | 04 protection; 05 processes; 03 faults | Define a narrow user/kernel ABI and service boundary | User code invokes approved services; invalid requests are rejected safely |
| 09 — Basic shell | 07 files; 08 ABI; console I/O | Provide a minimal interactive user program | A prompt accepts and reports a small documented command set |
| 10 — Networking | Working core and relevant drivers | Research and later implement a limited network path | Deferred; acceptance details require a separate decision |
| 11 — Advanced features | Evidence from earlier phases | Evaluate SMP, richer storage, security, and portability | Deferred; proposals must state value and dependencies |

## Dependency notes

- Early diagnostic output may use a minimal, temporary interface in phase 02; a general driver model belongs to phase 06.
- An existing loader may be preferable to writing a custom one. Phase 01 must decide its contract and document the tradeoff.
- Physical memory knowledge comes from the boot handoff. Virtual memory and page-fault handling precede reliable address-space isolation; heap allocation is introduced only when needed.
- A simple task runner may precede full user processes. A timer and safe context-switch design are prerequisites for preemption.
- The shell is a user-space goal and therefore follows process isolation, a syscall ABI, and minimal I/O. Networking is optional.

No phase advances solely because code exists: its planned evidence and documentation must also be reviewed. Track progress in [milestones](docs/09-project-management/milestones.md) and [progress tracker](docs/09-project-management/progress-tracker.md).
