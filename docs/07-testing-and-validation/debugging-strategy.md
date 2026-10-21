# Debugging strategy

When code is authorized, preserve a known-good QEMU profile and capture the earliest reliable observation: loader trace, serial output, debugger breakpoint, or fault report. Add complexity one stage at a time. For a failure, record the last confirmed stage, relevant registers and stack, image/symbol identity, and reproduction conditions.

Use symbolized traces for panic and exceptions, inspect page tables for memory faults, and inspect saved context for task failures. Treat timing-sensitive bugs carefully because a debugger can change scheduling. Avoid committing raw logs or dumps; summarize evidence in Markdown.

This is a future workflow, not a report of tools used. See [debugging environment plan](../02-development-environment/debugging-plan.md).
