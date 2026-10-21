# Open research questions

Each question remains unresolved until evidence and a [decision record](../09-project-management/decision-log.md) close it.

| Owner phase | Question | Evidence needed |
| --- | --- | --- |
| 01 | UEFI or legacy BIOS? Existing loader and which protocol? | Compare handoff contracts, QEMU support, learning cost, and licensing |
| 01 | What CPU state, stack, paging, and memory map does the loader guarantee? | Protocol specification and observed boot trace |
| 02 | Which early output and panic path are dependable? | Selected QEMU profile and failure demonstration |
| 03 | Which interrupt controller and timer should be introduced first? | Machine profile and documented interrupt routing |
| 04 | What virtual layout and frame allocator fit the handoff? | Reserved-memory analysis and mapping tests |
| 05 | Where does the first user address space begin? | Fault, privilege, and context-switch design |
| 06–07 | Which console and block devices minimize early driver work? | Device documentation and scoped prototypes |
| 08 | Which syscall entry mechanism and initial ABI? | CPU contract, calling convention, and failure cases |

Networking, SMP, real hardware, and broad compatibility are deferred questions rather than blockers for the first boot.
