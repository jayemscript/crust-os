# Vision

**Proposal:** crust-os should be a small, understandable operating system that makes the path from firmware handoff to user program observable. Its value is the sequence of well-explained experiments and reviewed design choices, not a claim of production suitability.

The first platform is proposed as x86-64 in QEMU because the architecture has detailed public references and repeatable emulation. The kernel model is proposed as monolithic so the first milestones can focus on hardware and ownership boundaries before introducing interprocess service boundaries.

**Success at this stage** is a coherent body of plans and research. A bootable system is a future milestone. See [objectives](objectives.md) and [scope](scope.md).
