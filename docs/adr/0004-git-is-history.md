# 0004 · Git is the history

**Status:** accepted · 2026-10-25

**Decision.** Every project is a git repository managed with gitoxide. A save is a commit. An applied Copilot run is a commit with `CADE-Copilot-Run`, `CADE-Model` and `CADE-Tokens` trailers. Version history is the commit log. Restoring creates a new commit, and branching is a git branch. Shipping pushes a branch and opens a pull request.

**Consequences.** No proprietary history store, and enterprises get review, retention and audit from GitHub for free.
