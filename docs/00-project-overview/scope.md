# Scope

## Initial scope

An educational x86-64 operating system in QEMU, with a small monolithic kernel, documented boot contract, exception handling, memory management, tasks and processes, limited devices, storage, syscalls, and a basic shell. All are planned, not implemented.

## Deferred scope

Real-hardware support, networking, SMP, graphics, broad device compatibility, secure boot, production security, and POSIX compatibility are outside the first milestones. Some appear as research or future phases; they are not prerequisites for a useful learning kernel.

**Assumption:** a single virtual CPU and a documented QEMU machine profile may be used initially. The precise profile, firmware, and boot protocol remain open in [research questions](../06-research/open-questions.md).
