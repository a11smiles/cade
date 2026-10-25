# 0006 · Feedback is GitHub issues filed as the user

**Status:** accepted · 2026-10-25

**Context.** Every CADE user already has a GitHub identity, because Copilot requires one.

**Decision.** Bug reports, feature requests and comments are GitHub issues on the CADE repository, created through the API with the signed-in user's token. The in-app dialog mirrors the repo's issue forms (`bug.yml`, `feature_request.yml`, `comment.yml`) and applies the same labels. `cade-feedback` builds and redacts the body. The user previews exactly what will be sent, and environment details are opt-out per item. `public_repo` scope is requested only the first time the user sends feedback.

Native GitHub issue types are an organization feature. While the repo is owned by a personal account, labels plus issue forms stand in for them. If the repo moves to an organization, add the Bug, Feature and Comment types and set them on creation (FEED backlog).

**Consequences.** No feedback backend to run. Issues are public, so redaction and preview are mandatory.
