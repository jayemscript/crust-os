# Rust fundamentals for kernel work

**Study before phase 02:** ownership, borrowing, lifetimes, enums, traits, error handling, generics, atomics, and the difference between core, alloc, and std. Bare-metal code cannot assume an OS-provided runtime or standard I/O. Rust's [Embedded Book](https://doc.rust-lang.org/embedded-book/intro/no-std.html) explains the \`no_std\` distinction; its examples may target other hardware and must not be copied as x86-64 designs.

## Practice goals

- Explain who owns a memory region and when references to it remain valid.
- State why volatile access, atomics, and ordinary memory access solve different problems.
- Write a safety argument for a hypothetical MMIO wrapper, including alignment and aliasing.
- Distinguish a compile-time guarantee from a hardware or bootloader guarantee.

This is a learning checklist, not an instruction to install tools or create code during foundation work.
