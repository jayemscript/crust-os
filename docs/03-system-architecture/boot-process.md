# Boot process

**Fact:** x86-64 machines can reach a kernel through different firmware and loader arrangements. UEFI specifies firmware services and a boot interface; a legacy BIOS route uses a different handoff model. The [UEFI specifications](https://uefi.org/specifications) and [AMD64 system programming manual](https://docs.amd.com/v/u/en-US/24593_3.44_APM_Vol2) are primary references. This project has selected neither route.

| Candidate | Learning value | Cost or risk to assess |
| --- | --- | --- |
| UEFI with an existing loader | Clear modern firmware interface; can focus on kernel handoff | Firmware image, protocol, and exit-boot-services contract must be understood |
| Legacy BIOS with an existing loader | Exposes historical PC boot flow | Mode transitions and legacy assumptions may distract from kernel goals |
| Custom bootloader | Teaches the entire path | Considerably expands phase 01 and creates extra correctness burden |

**Proposal:** prefer a documented existing loader for the first kernel, but decide only after comparing contracts and licensing. Phase 01 must specify CPU state at entry, stack, image placement, page tables, memory-map format and lifetime, device information, and failure reporting. The handoff's physical-memory ownership must be clear before phase 04.

**Open questions:** firmware route, loader/protocol, early output device, and whether the first kernel is entered in long mode. Record the resolution in [open questions](../06-research/open-questions.md) and the [decision log](../09-project-management/decision-log.md).
