#!/usr/bin/env python3
"""Add a design export as the next numbered version under design/exports/.

Usage: python3 scripts/design_export.py <export-folder> [--note "what changed"]

Never overwrites an existing version. Writes MANIFEST.json (sha256 per file),
updates INDEX.md and LATEST, and prints added/changed/removed files compared
with the previous version.
"""
import argparse, datetime, hashlib, json, pathlib, re, shutil, sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
EXPORTS = ROOT / "design" / "exports"


def digest(p: pathlib.Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def files_of(folder: pathlib.Path) -> dict:
    return {str(f.relative_to(folder)).replace("\\", "/"): digest(f) for f in sorted(folder.rglob("*")) if f.is_file() and f.name != "MANIFEST.json"}


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("source", type=pathlib.Path)
    ap.add_argument("--note", default="")
    a = ap.parse_args()
    if not a.source.is_dir():
        print(f"not a folder: {a.source}", file=sys.stderr)
        return 1
    versions = sorted(d for d in EXPORTS.iterdir() if d.is_dir() and re.match(r"v\d{4}-", d.name))
    last = versions[-1] if versions else None
    n = int(last.name[1:5]) + 1 if last else 1
    today = datetime.date.today().isoformat()
    dest = EXPORTS / f"v{n:04d}-{today}"
    if dest.exists():
        print(f"{dest.name} already exists; not overwriting", file=sys.stderr)
        return 1
    shutil.copytree(a.source, dest)
    new = files_of(dest)
    manifest = {"version": dest.name, "date": today, "note": a.note, "source": str(a.source), "files": new}
    (dest / "MANIFEST.json").write_text(json.dumps(manifest, indent=2) + "\n")

    if last:
        old = files_of(last)
        added = sorted(set(new) - set(old))
        removed = sorted(set(old) - set(new))
        changed = sorted(k for k in set(new) & set(old) if new[k] != old[k])
        print(f"Compared with {last.name}: {len(added)} added, {len(changed)} changed, {len(removed)} removed")
        for label, items in (("added", added), ("changed", changed), ("removed", removed)):
            for i in items:
                print(f"  {label:8} {i}")

    index = EXPORTS / "INDEX.md"
    lines = index.read_text().splitlines() if index.exists() else ["# Design export index", "", "| Version | Date | Files | Note |", "|---|---|---|---|"]
    row = f"| [{dest.name}]({dest.name}/) | {today} | {len(new)} | {a.note or '—'} |"
    head = 4 if len(lines) >= 4 else len(lines)
    lines.insert(head, row)
    index.write_text("\n".join(lines) + "\n")
    (EXPORTS / "LATEST").write_text(dest.name + "\n")
    print(f"Created design/exports/{dest.name}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
