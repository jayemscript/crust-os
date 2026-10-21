# System-call implementation plan

**Status: Planned documentation; implementation Not Started.** See [phase 08](../04-development-phases/phase-08-system-calls.md) and [security model](../03-system-architecture/security-model.md).

Specify the CPU entry/return mechanism, register ABI, syscall identifiers, argument sizes, errors, interrupt behavior, and handle ownership before implementation. Keep the first call set narrow: enough to prove user code can request a service and receive a checked result. Exact calls should be chosen against the shell's needs during phase 08.

Any user pointer crosses a trust boundary. Plan checked range and permission handling, bounded copying, and behavior if a fault occurs during access. After authorization, test allowed calls, unknown identifiers, invalid addresses, boundary-length buffers, and user process failure without kernel corruption. Version the ABI if later changes break callers.
