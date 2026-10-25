---
name: team-performance-review
description: "Review the last three hours of CADE agent logs, find concrete ways the agents can work better, update coaching notes, agent files and skills within guardrails, and return per-agent coaching for the architect to relay. Use for the scheduled three-hour team performance review."
---

# Team performance review

Owner: `performance-coach`. Dispatched by the architect every three hours.
The goal is a self-instructing team: each review turns evidence from the
logs into specific, lasting instructions.

## Inputs

The architect passes the review window (start and end, local time) and the
log locations. Sources:

- Copilot session logs or chat exports for the window, at the locations the
  architect names. If the host keeps no readable logs, say so and review only
  what is available; report the gap as a blocker.
- Claude Code transcripts, if Claude Code also ran in the window, at the
  locations the architect supplies, including subagent transcripts.
- The previous review in `.coaching/CHANGELOG.md` (create it on the first
  review).

Before reading any log, list every session log for this repository written
in the window, in both harnesses, whatever logs the architect names:
- Copilot CLI: `find ~/.copilot/session-state -maxdepth 2 -name events.jsonl
  -newermt '<window start>'`. Keep a file when its `session.start` or
  `session.resume` entry has `context.gitRoot` equal to the repository root.
  Subagent events are in the same file, keyed by `agentId`. `ask_user`
  answers are in `human_response.recorded` entries and tool results.
- Claude Code: enumerate JSONL transcripts modified in the window under the
  architect-supplied location, including subagent transcripts.

Review every log that has entries in the window, and list each one in the
changelog Sources with its in-window entry count.

Never read a log whole; they are large. Filter by timestamp to the window and
summarize with `jq` or `python3 -I` scripts written to a scratch directory
outside the repository. Stream every line of the file through the timestamp
filter; never cut it first with `tail -c`, `tail -n` or `head`. If any part
of the window goes unread, state the unread span as the first summary item.
Never copy secrets, credentials, tokens, BYOK keys or private user data out of
the logs.

## Steps

1. **Build the timeline.** For each agent active in the window: role,
   assignment, dispatch and handoff times, tool-call count, and outcome
   (handed off, rejected by QA, blocked, abandoned). Search message content
   in the full parsed text, never in line-filtered output.

   In Claude Code transcripts, user messages arrive three ways:
   - `user` entries;
   - AskUserQuestion answers in tool results;
   - `attachment` entries of type `queued_command` with
     `origin.kind == "human"`, which are messages sent mid-turn.

   Read all three.
2. **Measure.** For each role:
   - QA rejections and their root causes;
   - fix-and-recheck rounds per slice;
   - permission denials and how they were handled;
   - failed or repeated commands (including clippy and fmt failures caught
     only by QA) and long waits on cargo builds;
   - scope or ownership violations (files, crates, branches, processes, roles);
   - missing red-first evidence;
   - new duplicate types or code;
   - instructions that were ignored or misunderstood;
   - work that was redone or duplicated;
   - idle time while work was ready.

   Include the architect: dispatch clarity, routing delays, the QA queue,
   decisions made late, and context sent in prompts.
3. **Find patterns.** A finding needs evidence: a log line, a QA report or a
   repeated event. Prefer issues that recur or that cost a QA round. Ignore
   one-off noise.
4. **Decide the remedy at the right level:**
   - Role-specific habit: add it to `.coaching/<role>.md`.
   - Missing or wrong rule in a role definition: edit
     `.github/agents/<role>.agent.md`.
   - Missing or wrong method in a skill: edit `.github/skills/<skill>/SKILL.md`.
   - `AGENTS.md`, `.github/copilot-instructions.md`, `.github/instructions/`,
     settings, permissions or hooks: propose only. Don't edit.
5. **Apply the changes** within the guardrails below. Keep each one small and
   specific: "before handoff, run `cargo clippy --workspace --exclude
   cade-app --all-targets -- -D warnings` and report its exit code" beats
   "check for lint". Cite the evidence (review number, times, ids) in the
   edit, as a short parenthetical.
6. **Prune.** Remove or merge coaching items that are stale, done or
   contradicted. Keep each coaching file under about 60 lines. Check every
   item and edit from earlier reviews against `AGENTS.md` and the user's latest
   messages. Remove any that turns a workaround into a habit the user didn't
   ask for, and report it as your own error.
7. **Record it.** Add a numbered, dated entry to `.coaching/CHANGELOG.md` with:
   - the window;
   - sources and their in-window entry counts;
   - findings and their evidence;
   - files changed;
   - proposals;
   - how to revert.
8. **Return coaching** to the architect (see Output).

## Guardrails

- Never weaken, remove or contradict `AGENTS.md`, the testing rules (red-first,
  no skipped or weakened tests, no duplicate tests), integrity and security
  rules, role ownership, the user's decisions (ADRs) or approval gates.
  Coaching may only make them more specific or easier to follow.
- Never edit `AGENTS.md`, `.github/copilot-instructions.md`,
  `.github/instructions/`, `.github/workflows/`, `.github/ISSUE_TEMPLATE/`,
  harness settings, permission rules or hooks. Never grant tools or
  permissions. Propose these to the architect instead.
- Never edit product code, tests, `docs/`, `design/` or the backlog.
- Don't change this skill's Guardrails section or the `performance-coach`
  guardrails. Propose such changes to the architect, who asks the user.
- Make at most five edits to agent or skill files per review. Coaching files
  are not limited. Every edit cites its evidence.
- Name roles, not people. Don't include customer or private data.

## Output

Return, in this order:

1. **Summary:** the window, agents reviewed, and three to five key findings
   with numbers.
2. **Per-agent coaching:** for each role active or about to be dispatched, at
   most five imperative instructions, each tied to evidence. These are the
   messages the architect relays to running agents.
3. **Architect coaching:** the same, for the main session.
4. **Files changed,** with the reason for each.
5. **Proposals needing approval:** `AGENTS.md`, instructions, settings or
   guardrail changes.
6. **Metrics:** for trend tracking against the previous review.
