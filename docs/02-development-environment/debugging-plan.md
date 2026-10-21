# Debugging environment plan

Use a repeatable serial or equivalent early log channel, QEMU's debugging facilities, and a symbol-aware debugger such as GDB when implementation begins. Introduce each with the relevant phase, record versions and symbol format, and avoid assuming guest timing under a debugger matches normal execution.

Phase 01 needs evidence of where control stops. Phase 02 needs panic and symbol visibility. Phase 03 needs exception frames and interrupt return. Later memory and process work needs page-table, address-space, and task-state inspection.

See the [debugging strategy](../07-testing-and-validation/debugging-strategy.md). No debugging tools were installed or exercised for this document.
