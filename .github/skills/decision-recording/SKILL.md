---
name: decision-recording
description: "Record CADE architecture and product decisions as living per-topic records in docs/, and keep open questions current, with approval, rationale and issue traceability."
---

# Decision recording

Owner: document writer.

## Two kinds of decision

| Kind | Where | Lifetime |
|---|---|---|
| Architectural | `docs/adr/NNNN-<kebab-topic>.md`, one file per topic (e.g. `0002-loro-crdt.md`) | Living; changes slowly |
| Product | `docs/product/<kebab-topic>.md`, one file per topic (e.g. "Copilot usage limits"); create the folder on first use | Living; changes slowly |

An ADR holds only true architectural decisions. Copy, layout and per-issue
details belong in the issue, not in a record.

## Steps

1. Collect the decision, context, options considered, rationale, consequences,
   approval source (who decided, with a verbatim quote and the received time) and
   affected GitHub issues.
2. Classify it as architectural or product. If unclear, ask the architect;
   don't guess.
3. Find the existing topic record first. A change to an existing topic is an
   in-place update, never a new file: rewrite "Decision" and add a dated
   entry to "Change history" with what changed, why, who decided, and the quote.
   Example: moving from Vello to another renderer is a change-history entry in
   `0001-gpu-canvas.md`, not ADR 0008.
4. Create a new topic record only for a topic that has no record yet. Number
   ADRs with the next free four-digit number.
5. A decision with no approval evidence is marked "proposed" in its status and
   change-history entry. Use the time the decision was actually received.
6. When a decision resolves an entry in `docs/OPEN-QUESTIONS.md`, move the
   entry to "Resolved" and link the record. Keep the "See `docs/adr/`" pointer
   in `docs/ARCHITECTURE.md` and its overview current.
7. Send traceability updates (issues to relabel or unblock) to the project
   manager. Update contributor docs only with approved, verified behavior and
   commands.
8. Leave out secrets, personal information, private user content and agent chatter.

## Template

Keep the existing ADR style: a short title, status line, then bold-lead
paragraphs.

```markdown
# NNNN · <topic>

**Status:** accepted | proposed · YYYY-MM-DD

**Issues:** #N (EPIC-NNN), …

**Context.** <why this needed deciding>

**Decision.** <what is true now, in a few lines>

**Options considered.** <the alternatives and why they lost>

**Consequences.** <what follows, risks>

## Change history

| Date and time | Change | Justification | Decided by (quote) | Status |
|---|---|---|---|---|
```

Older records without "Options considered", "Issues" or "Change history" get
those sections added the first time they change.
