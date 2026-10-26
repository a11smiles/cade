---
name: domain-rule-testing
description: "Verify CADE's document model, CRDT merges, policy enforcement, Copilot change sets, git history, importers/exporters, generators and redaction with red-first, boundary, round-trip, concurrency and mutation tests."
---

# Domain-rule testing

Test approved backlog rules. Don't write a test that expects behavior listed in
`docs/OPEN-QUESTIONS.md`.

1. Map the assigned acceptance criteria to unit, integration (`crates/<crate>/tests/`),
   IPC and UI tests using the existing tools. Keep fixtures synthetic,
   deterministic and small; never commit a real user's project or file.
2. Pick the cases below that apply. Don't add unrelated scope.

| Area (epics) | Test techniques |
| --- | --- |
| Document model and CRDT (CORE) | Snapshot/open round trips, concurrent edits from two peers merged both ways, tree moves and cycles, older `FORMAT_VERSION` files, corrupt input returns a typed error |
| Undo and change sets (CORE, COPILOT) | A Copilot run stages on a fork, applies selectively, and undoes as one step; the base document is untouched until apply |
| Policy (POLICY, COPILOT) | Allow/deny matrix per rule; the stricter of org and Copilot policy wins; denied tool calls never execute; the cached policy is pinned to its commit |
| Copilot and BYOK (COPILOT) | Fake `ModelSource`; streaming, cancellation, provider errors, token metering and budget exhaustion; keys read from a fake keychain, never logged |
| Git and GitHub (GH, HIST) | Temp-dir repos; commit trailers; restore makes a new commit; device flow and API calls against a fake HTTP server, no live GitHub |
| Feedback (FEED) | Redaction of tokens, paths, emails and user content; the preview equals what is sent; labels match the issue forms |
| Importers and exporters (IMPORT, DIAGRAM, DECK, IO) | Minimal synthetic fixtures; malformed and hostile packages (zip bombs, macros, path traversal) are rejected; export → import round trips where both exist |
| Generators (APP, DATA, ARCH, SYS) | Golden output per dialect or target, compared exactly; reversible migrations; PII flags; missing-setting checks |
| Canvas (CANVAS) | Hit-testing and transforms as pure math tests; GPU rendering only behind a marked, separately run suite |
| UI (DS, ONB, app/ui) | Role/label queries, keyboard paths, both themes, IPC mocked at `ipc.ts` |

3. Test the concurrency: two peers editing the same node, a merge during an
   undo, a Copilot run applied while the user edits, a canceled background
   job, and a save interrupted by a crash (journal recovery). Check the
   committed document, file and git state, not just the returned value.
4. Use a controlled clock and seeded randomness. Never depend on wall time,
   network, the user's home directory, keychain or `~/.gitconfig`; tests use
   temp dirs and fakes.
5. Report pass, fail or blocked with the actual commands and evidence. A
   required test that fails intermittently is a defect, even if it predates
   the change. Report it as a finding with a proposed fix the first time you
   see it, not as an advisory.
6. Before handoff, developers run the affected suites unfiltered: `cargo test
   --workspace --exclude cade-app`, never a `-p` list or a test-name filter,
   because a change in one crate breaks the crates that use it. UI changes
   also run `pnpm --dir app/ui build`, `typecheck`, and the whole `test` and
   `test:e2e` suites, never a file, `-t` or `--grep` filter. Changes in
   `app/src-tauri` also run `cargo test -p cade-app`.
   - A refactor that adds a guard or any other new behavior needs its own
     test that fails first.
   - Run each new test and save its failing output to a log before the
     production edit that makes it pass, including helpers and guards you add
     while refactoring, and list each new test with its red log in the
     hand-back. A failing run captured afterward, by removing the code again,
     is retrospective red: label it so, never as red-first.
   - A red counts only when the new test ran and failed on its assertion, and
     the log names that test. A compile error or a `todo!()` panic is not a
     red: add the function's signature with a stub body that returns a wrong
     value, so the test compiles and fails on its assertion.
   - Before you say a changed test's assertions are unchanged, run `git diff
     -U0 <base> -- <changed test files>` and confirm that no added or removed
     line holds an assertion (`assert!`, `assert_eq!`, `assert_ne!`,
     `matches!`, `expect(`); otherwise list each old and new assertion.
7. When you fold duplicated tests into a shared helper, every value the old
   tests asserted must still be asserted. List the asserted values before and
   after the fold, and include both lists in the handoff.
   - The same applies when you rewrite an assertion for any other reason. Run
     at least one mutant that the old assertion caught against the new one,
     and show that it still fails. If keeping the assertion's strength would
     rename a test, ask the architect; never narrow the assertion to keep a name.
   - Splitting a test follows the same rule. List every assertion of the old
     test with the new test that carries it. An assertion that links two
     functions (state one leaves and another reads) can't be dropped; keep it
     in an integration test. Run the old test's own mutant against the new
     tests, as well as each new test's own mutant.
8. Before handoff, run mutants on your own new or changed guards. For each new
   condition, branch, `match` arm or loop exit, try at least these, one at a
   time:
   - negate the condition;
   - delete the guard;
   - move the boundary (for example `>=` to `>`, or 500 to 400).

   For each argument a new function passes on to another call, such as a
   cancellation token, a policy, a clock or a progress sink, also replace it
   with a default (`Policy::default()`, a fresh token, a system clock) and
   show that a test fails.

   Use `cargo mutants` scoped to the changed files if it is installed;
   otherwise mutate by hand. Run mutants in a scratch copy outside the
   repository with its own `CARGO_TARGET_DIR`, never in the live working
   tree that QA inspects and the architect stages from. After each manual
   mutant edit, confirm `diff` against the unmutated original shows the
   change before you run the tests. After restoring, confirm `cmp` shows the
   file identical to the original. Report a table of mutant, test that
   caught it, or "equivalent" with a reason, and name the copy's path. Also
   list every exit path of a retry or recovery loop and the test for each.
