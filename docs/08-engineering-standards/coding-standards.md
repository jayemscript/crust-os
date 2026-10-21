# Future coding standards

These standards apply when a specific implementation phase is authorized. There is no Rust source today.

- Favor small modules with one clear owner for hardware resources and memory.
- Express fallible operations as explicit results; avoid silent failure during boot.
- Keep unsafe blocks small. Each must document required invariants, who maintains them, and the consequence of violation.
- Do not create a safe wrapper around an unsafe operation unless every safe caller preserves those invariants.
- Distinguish ordinary, volatile, and atomic access. Do not use volatile as a concurrency primitive.
- Specify ABI, register, stack, and memory-layout assumptions at architecture boundaries.
- Introduce dependencies only for a named need and review their \`no_std\` suitability and maintenance state.
- Keep generated output and local tool stores out of the repository.

The relevant phase plan defines acceptance evidence; the [review checklist](review-checklist.md) is the final gate.
