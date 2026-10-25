#!/usr/bin/env python3
"""Validate the CADE backlog against backlog/CONVENTIONS.md.

Reports dangling depends_on ids, dependency cycles and format errors.
Exit 1 if anything is wrong.

Usage: python3 scripts/backlog_check.py [path/to/backlog]
"""
import re
import sys
from pathlib import Path

TYPE_TAGS = {"feature", "chore", "spike", "bug", "docs", "test"}
REQUIRED = ["id", "title", "tags", "epic", "priority", "depends_on"]
PRIORITIES = {"P0", "P1", "P2"}
FOLDER_RE = re.compile(r"^(\d\d)-([a-z0-9-]+)$")
FILE_RE = re.compile(r"^([A-Z]+)-(\d{3})-([a-z0-9-]+)\.md$")


def parse_list(v):
    v = v.strip()
    if not (v.startswith("[") and v.endswith("]")):
        return None
    inner = v[1:-1].strip()
    return [x.strip().strip("'\"") for x in inner.split(",") if x.strip()] if inner else []


def parse(path):
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        return None, "missing frontmatter"
    end = text.find("\n---\n", 4)
    if end < 0:
        return None, "unterminated frontmatter"
    fm = {}
    for line in text[4:end].splitlines():
        if not line.strip():
            continue
        if ":" not in line:
            return None, f"bad frontmatter line: {line!r}"
        k, v = line.split(":", 1)
        fm[k.strip()] = v.strip()
    body = text[end + 5:]
    return {"fm": fm, "body": body, "path": path}, None


def load(root):
    items, errors = {}, []
    for folder in sorted(p for p in root.iterdir() if p.is_dir()):
        if not FOLDER_RE.match(folder.name):
            errors.append(f"{folder.name}: folder name not NN-slug")
        for f in sorted(folder.glob("*.md")):
            it, err = parse(f)
            rel = f.relative_to(root)
            if err:
                errors.append(f"{rel}: {err}")
                continue
            it["folder"] = folder.name
            iid = it["fm"].get("id", "")
            if iid in items:
                errors.append(f"{rel}: duplicate id {iid} (also {items[iid]['path'].relative_to(root)})")
            items[iid] = it
    return items, errors


def validate(root):
    items, errors = load(root)
    for iid, it in items.items():
        fm, rel = it["fm"], it["path"].relative_to(root)
        for k in REQUIRED:
            if k not in fm:
                errors.append(f"{rel}: missing '{k}'")
        m = FILE_RE.match(it["path"].name)
        if not m:
            errors.append(f"{rel}: file name not EPIC-NNN-slug.md")
        elif f"{m.group(1)}-{m.group(2)}" != iid:
            errors.append(f"{rel}: file name does not match id {iid}")
        epic = fm.get("epic", "")
        if not iid.startswith(epic + "-"):
            errors.append(f"{rel}: id {iid} does not match epic {epic}")
        title = fm.get("title", "")
        if not title or len(title) >= 80:
            errors.append(f"{rel}: title empty or >= 80 chars ({len(title)})")
        tags = parse_list(fm.get("tags", ""))
        if tags is None:
            errors.append(f"{rel}: tags not a [list]")
        else:
            if epic.lower() not in tags:
                errors.append(f"{rel}: tags missing epic tag '{epic.lower()}'")
            types = [t for t in tags if t in TYPE_TAGS]
            if len(types) != 1:
                errors.append(f"{rel}: needs exactly one type tag, has {types}")
        if fm.get("priority") not in PRIORITIES:
            errors.append(f"{rel}: bad priority {fm.get('priority')!r}")
        deps = parse_list(fm.get("depends_on", ""))
        if deps is None:
            errors.append(f"{rel}: depends_on not a [list]")
            deps = []
        it["deps"] = deps
        for d in deps:
            if d not in items:
                errors.append(f"{rel}: dangling depends_on {d}")
            if d == iid:
                errors.append(f"{rel}: depends on itself")
        body = it["body"]
        if "## Description" not in body:
            errors.append(f"{rel}: missing ## Description")
        if "## Acceptance criteria" not in body:
            errors.append(f"{rel}: missing ## Acceptance criteria")
        else:
            ac = body.split("## Acceptance criteria", 1)[1].split("\n## ", 1)[0]
            n = len(re.findall(r"^- \[ \] ", ac, re.M))
            if not 3 <= n <= 7:
                errors.append(f"{rel}: {n} acceptance criteria (need 3-7)")
    # cycles
    state, stack, cycles = {}, [], []

    def visit(n):
        state[n] = 1
        stack.append(n)
        for d in items[n].get("deps", []):
            if d not in items:
                continue
            if state.get(d) == 1:
                cycles.append(stack[stack.index(d):] + [d])
            elif d not in state:
                visit(d)
        stack.pop()
        state[n] = 2

    sys.setrecursionlimit(10000)
    for n in sorted(items):
        if n not in state:
            visit(n)
    for c in cycles:
        errors.append("cycle: " + " -> ".join(c))
    return items, errors


def main():
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "backlog"
    items, errors = validate(root)
    for e in errors:
        print(e)
    print(f"{len(items)} items, {len(errors)} problems")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
