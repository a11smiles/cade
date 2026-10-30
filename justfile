# Explicit regeneration only; artifact-check tests never rewrite tracked files.
bindings:
    cargo run --quiet -p cade-feedback --example generate_feedback -- bindings app/ui/src/ipc/bindings.ts
