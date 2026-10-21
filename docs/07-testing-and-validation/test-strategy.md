# Validation strategy

**Status: plan only; no builds, boots, or tests have been run.** Each future phase must define the smallest observation that would satisfy its [acceptance criteria](../README.md#development-phases), plus at least one failure case.

Use three complementary levels when implementation is authorized:

1. Review contracts and unsafe invariants before execution.
2. Exercise pure parsers and policies with bounded fixtures where practical.
3. Observe boot, faults, interrupts, I/O, and user boundaries in a pinned QEMU profile.

Record tool and firmware versions, emulator settings, input fixtures, expected result, observed result, and limitations. A QEMU pass supports a claim about that virtual configuration; it does not establish real-hardware compatibility. Do not mark a phase Completed from compilation alone.
