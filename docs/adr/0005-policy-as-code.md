# 0005 · Organization policy as code

**Status:** accepted · 2026-10-25

**Decision.** Admins keep `cade/policy.yml` in their org's `.github` repository and change it through pull requests reviewed by CODEOWNERS. Clients fetch it on sign-in and cache it pinned to a commit. `cade-policy` is the only place decisions are made. CADE also honors Copilot org policies the SDK exposes, and the stricter rule wins.

**Consequences.** No CADE server is needed. Admin changes are auditable in git.
