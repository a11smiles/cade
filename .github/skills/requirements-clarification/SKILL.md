---
name: requirements-clarification
description: "Turn CADE backlog issues into approved, testable acceptance criteria. Use for unclear behavior, conflicting sources, or issues that need splitting."
---

# Requirements clarification

Owner: business analyst.

1. Read the issue, its blocked-by issues, its `**Design:**` export and the related
   ADRs. Refer to work by issue number and backlog id (`#5` / `CORE-002`).
2. Identify the actors (user, organization admin, Copilot agent, reviewer),
   resources, preconditions, allowed and denied actions, state transitions,
   side effects and what users see on success and failure.
3. Compare the sources (issue, design export, `design/system/README.md`,
   ADRs) and check `docs/OPEN-QUESTIONS.md`. Don't assume one source
   automatically wins in a contradiction. The fixed technical decisions in
   `docs/adr/` are not up for clarification.
4. Ask only questions that change behavior. Give the options, examples and
   consequences, and record the product owner's actual answer. If you can't ask
   the user directly, return the questions to the architect, who asks the user.
5. Cover the edge cases that apply: policy denial and the stricter of org and
   Copilot policy; Copilot vs BYOK; offline and signed-out; token budget
   exhausted; concurrent edits and merges; undo of a whole Copilot run; large
   documents; older `.cade` versions; macOS, Windows and Linux differences;
   dark and light themes; keyboard-only use; GHES and GHE.com hosts. Use
   synthetic examples.
6. Keep policy, Copilot permission, shipping and feedback-redaction rules
   consistent across issues. UI wording must not redefine what `cade-policy`
   decides.
7. If an item is bigger than 1–5 days or has more than 7 acceptance criteria,
   propose a split.

Output: issue reference, a matrix of actors and rules, acceptance examples
(Given/When/Then where useful, including denial and recovery cases), and separate
lists of confirmed requirements, approved interim behavior, open questions and
excluded scope. Write approved changes into the issue yourself, tell the project
manager when readiness or dependencies change, and send every decision, with its
approval source, to the document writer.
