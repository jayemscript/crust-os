# Rust OS development research

**Fact:** \`no_std\` avoids depending on the standard-library runtime, while the core library remains available. Dynamic allocation requires an allocator and deliberate initialization; see the [Embedded Rust Book](https://doc.rust-lang.org/embedded-book/intro/no-std.html). This alone does not supply kernel entry, paging, interrupt support, or a boot image.

**Proposal:** choose the boot protocol and image format before adding a Rust target, linker configuration, or Cargo manifest. Then document entry ABI, panic path, global initialization, allocator requirements, and unsafe boundaries. Keep architecture-specific assembly small and explained.

**Open questions:** stable versus unstable Rust facilities, target specification, linker choice, loader integration, and symbol/debug information. Resolve these in phases 01 and 02, using versioned tool documentation. No Rust toolchain has been exercised for this project.
