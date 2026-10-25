---
name: backlog-management
description: "Create, triage and remediate CADE GitHub issues: [EPIC-NNN] ids, labels, milestones, native blocking dependencies, the CADE backlog project, readiness and status. Use for new work, defects, app feedback, backlog triage or cleaning up issues."
---

# Backlog management

Owner: project manager, for labels, dependencies, milestones, project status and
closing. The business analyst writes and updates requirement content. Issues live
in `a11smiles/cade`; the "CADE backlog" user project (#4, owner `a11smiles`)
tracks them.

The backlog was imported from Markdown files that are no longer in the tree
(they're in git history at `a96561f`). GitHub issues are the only source of
truth; their `**Source:**` links point at that history.

## Ids and titles

- Backlog items are titled `[EPIC-NNN] Imperative title`, under 80 characters
  after the id (for example `[CORE-011] Route all edits through a validated
  operations API`). The body starts with `<!-- cade-backlog-id: EPIC-NNN -->`.
- Ids are zero-padded, unique and never reused. A new item takes the next
  free number in its epic: search all issues, open and closed, for
  `"[EPIC-" in:title` and add one to the highest.
- Epic prefixes: FOUND, DS, CORE, CANVAS, COPILOT, APP, IMAGE, DECK, DOC,
  MOTION, DIAGRAM, ARCH, DATA, SYS, GH, POLICY, IMPORT, HIST, ONB, FEED, QA, REL.
- Refer to issues by `#number` and backlog id together (`#5` / `CORE-002`).
- Issues filed from the forms or the app (`[Bug]`, `[Feature]`, comments) keep
  their title until triage turns them into a backlog item; then the business
  analyst gives them an id.

## Labels and milestones

- **Type (exactly one):** `feature`, `chore`, `spike`, `bug`, `docs` or `test`.
  Form issues arrive as `bug`, `enhancement` or `feedback`; triage adds the
  backlog type once the item is accepted.
- **Epic (both):** `epic:<PREFIX>` and the lowercase epic tag (`core`,
  `canvas`, …).
- **Priority (exactly one):** `priority:P0` (needed for any usable build),
  `priority:P1` (needed for 1.0) or `priority:P2` (after 1.0), with the
  matching milestone `P0`, `P1` or `P2`.
- **Delivery:** `milestone:M1` for the M1 slice only.
- **Surface (as needed):** `ui`, `rust`, `ts`, `windows`, `macos`, `linux`,
  `security`, `a11y`, `perf`, `policy`.
- **`needs-decision`:** blocked on an entry in `docs/OPEN-QUESTIONS.md`. Create
  the label if it doesn't exist yet.
- Feedback labels (`feedback`, `needs-triage`, `from-app`, `security`) are
  defined in `.github/labels.yml`. `from-app` marks issues CADE filed for a user.
- Never add stage or workflow labels such as `qa`, `in-progress` or `review`.
  Ask before creating any other new label.

## Status

Work out status from GitHub instead of tracking it in labels, and keep the
project's Status field in step:

| Status | Meaning |
| --- | --- |
| blocked | Has an open blocked-by issue or the `needs-decision` label |
| ready | Open, not blocked, no assignee or linked PR |
| in progress | Has an assignee or a linked open PR |
| done | Closed as completed |

Ready order: `milestone:M1` items first, then P0, P1, P2, each in blocked-by
order and then by issue number.

## New issues

1. Search open and closed issues first. If a match exists, add to it instead.
2. Give it the next free `[EPIC-NNN]` id and an imperative title.
3. Write the body in the imported format: the id comment; `**Backlog ID:**`,
   `**Epic:**` and `**Priority:**` lines; an optional `**Design:**` line
   linking `design/exports/vNNNN-…/<Screen>.dc.html`; a `**Delivery:**` line
   for M1 items; `## Description` (2–6 sentences that name the crate(s) or
   `app/ui` area); `## Acceptance criteria` (3–7 checkboxes, each verifiable
   by a test or a demo step); `## Dependencies` (`- Depends on #N (EPIC-NNN)`
   or "None."); and an optional `## Notes`. New items have no `**Source:**`
   line. Size items
   for 1–5 days; split anything bigger. If the requirements aren't clear yet,
   get them from the business analyst before calling the issue ready.
4. For a bug, include reproduction steps, expected and actual behavior, OS and
   CADE version, and evidence. QA defects come with these. Never paste
   unredacted user content, tokens or logs.
5. Add labels, milestone and the project item. Add blocked-by relationships
   only for real dependencies, using the blocking issue's database ID, not its
   number. If one issue just mentions another, that's a related reference, not
   a blocker.

Use the GitHub CLI: `gh issue list|view|create|edit|close`, `gh label`,
`gh project item-add 4 --owner a11smiles --url <issue url>`, and `gh api` for
what the CLI lacks, e.g. the blocking issue's database ID
(`gh api repos/a11smiles/cade/issues/<n> --jq .id`) and blocked-by
relationships (`gh api -X POST repos/a11smiles/cade/issues/<n>/dependencies/blocked_by -F issue_id=<id>`).

## Triage

Go through new, `needs-triage` and blocked issues and:

- Apply labels from the form answers (the "Which output type?" dropdown maps
  to the epic tag).
- Check that each one has labels, a milestone, acceptance criteria and correct
  dependencies.
- For `from-app` issues, check that nothing private slipped past redaction.
  If it did, report it to the architect at once; don't quote it anywhere.
- When an open question is resolved or a decision supersedes an earlier one, update
  the affected issues, remove `needs-decision` and link the decision record.
- Report what's ready, what's blocked and why, and what's waiting on a decision.

## Remediation

Fix issues that are:

- **incomplete** (no labels, milestone or dependencies; send missing acceptance
  criteria to the business analyst),
- **stale** (pointing to a superseded decision or design export), or
- **duplicates**.

Keep the original description and any edits people made by hand. Add corrections
as a clearly marked update section or a comment. Never silently rewrite the
requirements. Close a duplicate only with a link to the issue it duplicates and
with user approval.
