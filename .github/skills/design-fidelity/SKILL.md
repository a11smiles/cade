---
name: design-fidelity
description: "Implement or review CADE screens against the issue's design export, the design-system tokens and components, both themes, desktop window sizes and accessibility criteria."
---

# Design fidelity and accessibility

Used by the designer (specs), the TypeScript developer (UI code) and the QA analyst
(acceptance evidence).

1. Read the affected issue, its acceptance criteria and its `**Design:**` export
   (`design/exports/vNNNN-…/<Screen>.dc.html` and `<Screen>.light.dc.html`).
   If the issue names an older export than `design/exports/LATEST`, check
   `design/exports/INDEX.md` for changes to that screen and ask the architect
   which version applies. Cover the loading, empty, error, success,
   policy-blocked and Copilot-staged states that apply.
2. Map every element to tokens from `design/system/tokens.json` and components
   in `app/ui/src/components/`. Match the typography, spacing, color roles,
   iconography and copy rules in `design/system/README.md`, and the existing
   screens.
3. Check screenshots at window sizes 1100 × 700 (minimum), 1440 × 900
   (default) and 1920 × 1080, in dark and light themes, captured with
   Playwright (Chromium) against `pnpm --dir app/ui dev`; open and inspect the
   PNGs. Check long names, paths and SHAs, wrapping content, panel resizing
   and overflow. When a change depends on the native shell (window chrome,
   canvas compositing, menus), also check it in `cargo run -p cade-app` and
   say which OS you used.
   Reviewers (designer, QA) run servers from a scratch copy of the working tree
   outside the repository (rsync, excluding `node_modules`, `dist` and
   `target`) on their own ports. Every role saves `$!` at launch, stops only
   that PID, never uses `pkill -f`, and leaves the tree's `git status` as it
   found it.
4. Operate every control with the keyboard. Check labels, focus visibility and
   order, focus trapping in dialogs and where focus returns, menus and the
   command palette, errors, reduced motion, forced-colors mode and hit areas
   of at least `control-md`. Measure contrast in both themes instead of
   assuming the tokens pass.
   Before handoff, the developer measures contrast for every state the change
   adds, including error, placeholder, disabled and loading text: at least
   4.5:1 for body text and 3:1 for icons and large text. It also proves each
   message is visible, using a bounding-box or hit-test check, not just DOM
   presence. List every measurement in the handoff.
5. Run any existing automated accessibility checks. Report serious and critical
   findings, and any conflict with the exact approved values. Get approval before
   changing the design system.
6. For each discrepancy, record expected and actual behavior, window size,
   theme, state and reproduction steps. Use synthetic data. Don't publish
   screenshots externally without authorization.

Output: comparison evidence, discrepancies and acceptance status. A matching
screenshot doesn't prove that keyboard, policy or document behavior is correct.
