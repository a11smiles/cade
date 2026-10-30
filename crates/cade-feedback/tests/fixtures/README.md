# Feedback fixtures

`legacy-feedback.json` covers migration of synthetic heading/text payloads.
`form-schema-cases.json` contains one valid control and six invalid forms;
the tests check both rejection and the location of each intended violation.

`github-issue-forms.json` is the unmodified SchemaStore/schemastore source at
commit `64cc4a9ee27111df7a6a68eb64451690234a12ba`,
path `src/schemas/json/github-issue-forms.json`, retrieved October 26, 2026.
It is third-party SchemaStore material, **not an official GitHub schema**.
Its Apache-2.0 terms are supplied in the repository's root `LICENSE`;
attribution is in root `NOTICE`. The upstream license is Apache-2.0.
No separate license variant or schema modifications are introduced here.

Expected source SHA-256:
`c2722dbf00334ce4fdeffa960b8c9047caf4f1cbb8f3809663f4d604b1d3ae76`
(83,802 bytes). Upstream `LICENSE` SHA-256:
`cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`.
Keep the source bytes intact; update provenance explicitly if replacing it.

`form_schema` uses `jsonschema = 0.56.0` as a dev-only dependency with default
features disabled and `.offline()`. The vendored `$ref` values are local
fragments. Schema identifiers, documentation URLs and the built-in Draft 7
meta-schema identifier do not authorize network retrieval.
These checks do not certify hosted GitHub acceptance or replace answer validation.

Run from the repository root, using an executor-owned `CARGO_TARGET_DIR`:

```sh
cargo build -p cade-feedback --example generate_feedback
cargo test -p cade-feedback --test form_schema
cargo test -p cade-feedback --test preview_command
```

The private runner invokes `$CARGO_TARGET_DIR/debug/examples/generate_feedback`
(`.exe` on Windows) with `preview`, writes one Feedback JSON object to stdin,
and closes stdin. Stdout is one JSON value: `{"ok": IssueDraft}` or
`{"error":{"Message":"..."}}`. Validation calls `Feedback::to_issue`; no service
is called. Malformed input receives a fixed message without deserializer details.
The runner is test infrastructure, not an IPC command or production domain type.
`bindings` without an output path and `forms` print generated artifacts to
stdout. Bindings also accepts an explicit output path: it writes the complete
artifact without printing it and fails if the write fails. Transport tests
compare file output with complete stdout bytes and exercise write failure using
temporary fixtures. See [contributor guidance](../../../../CONTRIBUTING.md#maintaining-feedback-contracts)
for explicit regeneration and the root recipe's tooling prerequisite.
Ordinary tests never regenerate tracked artifacts; full-artifact drift and
autocrlf/line-ending checks remain in `tests/generated.rs`. The commands above
are focused checks, not the full workspace acceptance gate.
