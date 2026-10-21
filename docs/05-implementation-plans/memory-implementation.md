# Memory implementation plan

**Status: Planned documentation; implementation Not Started.** See [phase 04](../04-development-phases/phase-04-memory-management.md) and the proposed [memory model](../03-system-architecture/memory-model.md).

## Interfaces to define

The boot memory map needs region type, bounds, and lifetime rules. A physical allocator should yield only usable, unowned frames. A mapping interface should express virtual page, physical frame, permissions, owner, and invalidation obligations. A heap allocator is an optional later layer.

## Sequence after authorization

1. Validate and normalize the handoff map; reserve image, stack, firmware, boot data, and MMIO.
2. Implement one simple frame allocator with explicit exhaustion behavior.
3. Build mapping and unmapping operations and record page-table ownership.
4. Add heap allocation only for a concrete consumer.

Test boundary regions, overlapping or malformed map entries, exhaustion, protection faults, and mapping lifetime. Review unsafe code for aliasing, alignment, initialization, and concurrent access. The selected paging mode and virtual layout must be entered in the decision log.
