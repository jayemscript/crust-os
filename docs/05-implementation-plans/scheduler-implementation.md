# Scheduler implementation plan

**Status: Planned documentation; implementation Not Started.** See [phase 05](../04-development-phases/phase-05-process-management.md).

## Minimum model

Define task identity, runnable/blocked/finished states, saved CPU context, owned stack, and cleanup rule. Keep a switch mechanism separate from a first simple policy such as round-robin; the exact policy is a phase decision. Cooperative switching can establish correctness before timer preemption.

## Sequence after authorization

Specify the context ABI and stack layout; implement switching between two kernel tasks; then add timer-driven scheduling if exception and lock behavior are understood. Add process address spaces and privilege transitions only with a documented isolation contract.

## Evidence and risks

Observe progress for multiple tasks, termination and resource reclamation, and behavior under timer stress. Audit use-after-free stacks, lock acquisition in interrupt context, fairness claims, and address-space switch invalidation. Do not describe task switching as process isolation.
