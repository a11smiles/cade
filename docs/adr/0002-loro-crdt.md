# 0002 · Loro CRDT as the document model

**Status:** accepted · 2026-10-25

**Context.** Real-time co-editing is a requirement. So is merging a Copilot run, or a teammate's branch, into a design without conflicts. Layer trees need conflict-free moves.

**Decision.** Every project is a Loro document. The layer tree is a Loro movable tree, and node properties live in each tree node's metadata map. On disk a `.cade` file is the Loro snapshot, with a JSON mirror for readable PR diffs. Git still stores history (ADR 0004).

**Consequences.** Undo and redo use Loro's undo manager. File-format versioning wraps the snapshot (CORE backlog). Automerge was the alternative; Loro won on its movable tree and performance.
