# Toolchain plan

**Proposal:** use a documented Rust release and matching target support when implementation starts. Decide whether an existing target is sufficient or a custom target specification is required only after the boot protocol and linker requirements are known. Rust \`no_std\` removes the standard-library runtime, but the kernel still needs deliberate entry, panic, allocation, and linking decisions.

Before adding configuration, record rustc/Cargo versions, target triple or specification, linker choice, binary format, and any unstable feature requirements. Prefer stable facilities where feasible. Check LLVM inspection tools against the output format. Revalidate these choices when versions change.

No Cargo manifest, target file, linker script, or command is part of the current repository. See [Rust OS research](../06-research/rust-os-development.md).
