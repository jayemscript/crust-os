# Computer architecture learning path

Study execution modes, registers, calling conventions, interrupts, exceptions, page translation, caches, and device I/O before their corresponding phases. The [AMD64 system programming manual](https://docs.amd.com/v/u/en-US/24593_3.44_APM_Vol2) is a primary reference for privilege, paging, and exception behavior; consult the revision applicable when implementation begins.

## Sequence

1. Trace reset and firmware to loader and kernel handoff conceptually.
2. Explain long mode, privilege levels, descriptor tables, and interrupt return.
3. Walk a virtual address through translation and permissions.
4. Compare port I/O, MMIO, interrupts, and polling.

**Open question:** which QEMU machine, firmware, and devices will form the initial reproducible platform? Track the answer in [open questions](../06-research/open-questions.md).
