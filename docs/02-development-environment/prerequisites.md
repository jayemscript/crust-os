# Future development prerequisites

No tools are being installed or configured in this foundation task. When phase 01 is authorized, establish a reproducible host environment and record versions.

| Proposed tool | Purpose | First likely phase |
| --- | --- | --- |
| Rust and Cargo | Compile a \`no_std\` kernel once a target and boot contract exist | 01–02 |
| Rust target support or custom target specification | Define CPU, ABI, linker, and runtime assumptions | 01–02 |
| QEMU system emulator | Run an x86-64 virtual machine | 01 |
| Firmware image, if UEFI is selected | Supply the chosen firmware path | 01 |
| LLVM utilities | Inspect object files and symbols | 02 |
| GDB | Diagnose boot and exception failures | 01–03 |
| Git | Review small, auditable changes | Already present |
| Assembly tooling, if needed | Produce strictly necessary architecture glue | When required |

Check host/guest architecture, versions, target support, firmware availability, and debugger compatibility at the relevant phase. Do not infer compatibility from this list.
