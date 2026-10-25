//! `scan` over an exported Spark repo laid out on disk, the way the importer reads one.

use cade_io_spark::scan;
use std::fs;
use std::path::Path;

fn write(root: &Path, rel: &str, text: &str) {
    let p = root.join(rel);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(p, text).unwrap();
}

fn sources(root: &Path) -> Vec<(String, String)> {
    let mut out = vec![];
    let mut stack = vec![root.join("src")];
    while let Some(dir) = stack.pop() {
        for e in fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                stack.push(p);
            } else {
                let rel = p.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
                out.push((rel, fs::read_to_string(&p).unwrap()));
            }
        }
    }
    out.sort();
    out
}

#[test]
fn scan_exported_repo_on_disk_lists_llm_and_kv_files() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    write(
        root,
        "package.json",
        r#"{"name":"claims","dependencies":{"@github/spark":"^0.39.0","react":"^19.0.0"}}"#,
    );
    write(
        root,
        "src/App.tsx",
        "import { useKV } from '@github/spark/hooks';\nconst [claims] = useKV('claims', []);",
    );
    write(
        root,
        "src/lib/summarize.ts",
        "export const s = (t: string) => spark.llm(`Summarize ${t}`);",
    );
    write(root, "src/lib/legacy.ts", "const r = await llm(prompt, 'gpt-4o');");
    write(root, "src/components/Badge.tsx", "export const Badge = () => null;");

    let files = sources(root);
    let refs: Vec<(&str, &str)> = files.iter().map(|(p, t)| (p.as_str(), t.as_str())).collect();
    let s = scan(&fs::read_to_string(root.join("package.json")).unwrap(), &refs).unwrap();

    assert!(s.is_spark);
    assert_eq!(s.llm_calls, vec!["src/lib/legacy.ts", "src/lib/summarize.ts"]);
    assert_eq!(s.kv_uses, vec!["src/App.tsx"]);
}
