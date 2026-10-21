# Research references and evidence rules

Use a source for the claim it actually supports. Record title, publisher, revision or date, relevant section, and the claim drawn from it. Primary specifications take precedence for hardware contracts; tool documentation governs tool behavior. A tutorial can aid learning but should not silently become a specification.

## Primary starting references consulted for this foundation

| Source | Relevant subjects |
| --- | --- |
| [AMD64 Architecture Programmer's Manual, Volume 2](https://docs.amd.com/v/u/en-US/24593_3.44_APM_Vol2) | Long mode, privilege, interrupts, paging, system programming |
| [Intel 64 and IA-32 software developer manuals](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) | Alternative primary account of x86-64 modes, protection, paging, and exceptions |
| [UEFI specifications](https://uefi.org/specifications) | Firmware services and OS loader interface |
| [Rust Embedded Book: \`no_std\`](https://doc.rust-lang.org/embedded-book/intro/no-std.html) | Bare-metal runtime and core/alloc/std distinctions |
| [Rust Reference](https://doc.rust-lang.org/reference/) | Language rules requiring precision in unsafe code |
| [QEMU system emulator documentation](https://www.qemu.org/docs/master/system/index.html) | Virtual-machine operation and device models |
| [OASIS VirtIO 1.2 specification](https://docs.oasis-open.org/virtio/virtio/v1.2/virtio-v1.2.html) | One possible virtual-device family; no VirtIO device is selected yet |

These references support planning, not claims that a particular boot path or tool version works in this repository. Recheck current revisions and exact sections before implementation.

## Topic map

| Research subject | Project explanation and unresolved work |
| --- | --- |
| x86-64, privilege, IDT, paging | [Architecture learning path](../01-learning-path/computer-architecture.md), [interrupt phase](../04-development-phases/phase-03-interrupts.md), [memory model](../03-system-architecture/memory-model.md) |
| UEFI, BIOS, bootloader alternatives | [Boot comparison](../03-system-architecture/boot-process.md) and [phase 01](../04-development-phases/phase-01-bootloader.md) |
| Rust \`no_std\`, unsafe pointers, heap | [Rust OS research](rust-os-development.md), [memory and pointers](../01-learning-path/memory-and-pointers.md), [memory plan](../05-implementation-plans/memory-implementation.md) |
| Context switching, scheduling, kernel/user boundary | [Scheduler plan](../05-implementation-plans/scheduler-implementation.md), [security model](../03-system-architecture/security-model.md), [phase 05](../04-development-phases/phase-05-process-management.md) |
| Syscalls, drivers, file systems, networking | [Syscall plan](../05-implementation-plans/syscall-implementation.md), [driver plan](../05-implementation-plans/drivers-implementation.md), [file-system plan](../05-implementation-plans/filesystem-implementation.md), [networking phase](../04-development-phases/phase-10-networking.md) |
| Kernel security and memory safety | [Security model](../03-system-architecture/security-model.md) and [memory-safety strategy](../07-testing-and-validation/memory-safety.md) |

## Claim labels

- **Fact:** supported by a cited source or direct observation, with scope stated.
- **Proposal:** a design option recommended for review.
- **Assumption:** a temporary condition used for planning.
- **Open question:** unresolved choice or missing evidence with an owning phase.
