# Open questions

Requirement conflicts and decisions that block specific issues. Work affected by
an open entry waits; everything else continues. The business analyst adds
entries, and the document writer moves them to "Resolved" with a link to the
decision record.

## Open

| Id | Question | Blocks | Options | Raised |
|---|---|---|---|---|
| OQ-550 | Does “Terraform map keys are quoted” require every key to be quoted, despite “Golden files for ordinary values are unchanged” and the narrower original Expected text? | Only #550's broader quoting criterion/full acceptance; not its approved `for` correction or unrelated fixes | Quote all keys with an explicitly approved ordinary-golden change, or quote syntax-required keys while preserving ordinary output | 2026-10-26 |

### OQ-550 — Terraform map-key quoting

**Status:** open; full quoting choice awaits the product owner.
**Source:** [#550](https://github.com/a11smiles/cade/issues/550), original criteria:

> Terraform map keys are quoted

> Golden files for ordinary values are unchanged

The original Expected section states:

> Bicep escapes `'` as `\'`; Terraform escapes `"`, `\` and `${`, and quotes keys that aren't plain identifiers.

Quoting every key would change ordinary goldens that retain bare plain identifiers.
The approved narrow fix quotes parser-significant `for` (and nonplain keys),
preserves ordinary output, and has bounded QA approval. It does not silently
resolve the all-key interpretation or weaken either original criterion.
See [Generator compatibility](ARCHITECTURE.md#generator-compatibility).

**Approval boundary:** product-owner instruction received
2026-10-26 at 17:14:12.058 −07:00: “Narrowforfixapproved, fullquotingchoicependinguser”.
Neither broader option is accepted. Missing database, hosted/native, package or
monitoring evidence is tracked per issue, not as additional open decisions here.

## Resolved

| Id | Question | Decision | Record | Resolved |
|---|---|---|---|---|
| Feedback contract | Earlier briefs left payload shape, metadata ownership, labels, copy and the environment ID unresolved. | Canonical keyed answers with legacy migration; YAML authority and Rust-derived declarations; legacy plus required type labels; approved copy; named `version` → `app_version` migration. No redundant approval needed. | [Feedback contract](ARCHITECTURE.md#feedback-contract), [Change history](ARCHITECTURE.md#change-history) | 2026-10-26 |
