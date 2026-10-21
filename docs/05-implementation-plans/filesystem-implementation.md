# File-system implementation plan

**Status: Planned documentation; implementation Not Started.** See [phase 07](../04-development-phases/phase-07-file-system.md).

The first file layer should be read-only unless a specific learning goal requires writes. Choose a documented image format and known fixture, then separate block reads, metadata parsing, path/name handling, and file handles. Parsing must check offsets, lengths, arithmetic overflow, and malformed entries before constructing references.

After authorization, prove known reads and predictable failures for missing or malformed data. A writable file system needs a separate consistency and recovery design; a successful in-memory update is not evidence of durable storage. Record exact image and virtual block device assumptions with test observations.
