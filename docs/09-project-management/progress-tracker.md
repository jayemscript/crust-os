# Progress tracker

## Status vocabulary

| Status | Meaning |
| --- | --- |
| Not Started | No implementation or phase-specific work has begun |
| In Research | Evidence is being gathered; decisions are open |
| Planned | A reviewable design exists; implementation has not begun |
| In Progress | Authorized work is under way |
| Blocked | A named dependency prevents progress |
| Completed | Documented acceptance evidence exists |

## Current phase status

| Phase | Status | Evidence or next gate |
| --- | --- | --- |
| 00 Foundation | Completed | Documentation foundation created; no OS implementation |
| 01 Boot integration | Not Started | Select handoff and QEMU profile |
| 02 Kernel | Not Started | Phase 01 evidence |
| 03 Interrupts | Not Started | Kernel diagnostics |
| 04 Memory | Not Started | Memory map and fault diagnostics |
| 05 Processes | Not Started | Timer and memory isolation |
| 06 Drivers | Not Started | Required device selection |
| 07 File system | Not Started | Block I/O |
| 08 System calls | Not Started | User isolation |
| 09 Shell | Not Started | ABI and console |
| 10 Networking | Not Started | Future decision |
| 11 Advanced features | Not Started | Future decision |

For every status change, link the relevant phase evidence or decision. Planned documentation does not make a phase Planned unless its major decisions are resolved.
