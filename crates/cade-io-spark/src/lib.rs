//! Import of exported GitHub Spark apps.
//!
//! Spark closed to new apps on 2026-08-04 and its `llm()` helper stopped
//! working when GitHub Models retired on 2026-07-30. Exported apps are
//! ordinary React + TypeScript repos. This module recognizes one and lists
//! what needs converting; the conversion itself is IMPORT backlog work.

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SparkScan {
    pub is_spark: bool,
    /// Files that call the retired `llm()` helper; rewired to the Copilot SDK.
    pub llm_calls: Vec<String>,
    /// Files that use `useKV`; mapped to Data model entities.
    pub kv_uses: Vec<String>,
}

/// `package_json` is the repo's package.json; `sources` are (path, contents).
pub fn scan(package_json: &str, sources: &[(&str, &str)]) -> Result<SparkScan, serde_json::Error> {
    let pkg: Value = serde_json::from_str(package_json)?;
    let has = |section: &str| pkg.get(section).and_then(|d| d.get("@github/spark")).is_some();
    let is_spark = has("dependencies") || has("devDependencies");
    let mut out = SparkScan {
        is_spark,
        ..Default::default()
    };
    for (path, text) in sources {
        if text.contains("llm(") || text.contains("spark.llm") {
            out.llm_calls.push(path.to_string());
        }
        if text.contains("useKV") {
            out.kv_uses.push(path.to_string());
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn recognizes_a_spark_export() {
        let pkg = r#"{"dependencies":{"@github/spark":"^0.39.0","react":"^19.0.0"}}"#;
        let s = super::scan(
            pkg,
            &[
                ("src/App.tsx", "const [items] = useKV('items', [])"),
                ("src/ai.ts", "await spark.llm(prompt)"),
            ],
        )
        .unwrap();
        assert!(s.is_spark);
        assert_eq!(s.llm_calls, vec!["src/ai.ts"]);
        assert_eq!(s.kv_uses, vec!["src/App.tsx"]);
    }

    use super::*;

    #[test]
    fn scan_spark_in_dev_dependencies_is_spark() {
        let s = scan(r#"{"devDependencies":{"@github/spark":"0.40.1"}}"#, &[]).unwrap();
        assert_eq!(
            s,
            SparkScan {
                is_spark: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn scan_plain_react_repo_is_not_spark() {
        let s = scan(r#"{"dependencies":{"react":"^19.0.0"},"devDependencies":{"vite":"^6"}}"#, &[]).unwrap();
        assert!(!s.is_spark);
    }

    #[test]
    fn scan_spark_named_outside_dependency_sections_is_not_spark() {
        let s = scan(r#"{"name":"@github/spark","peerDependencies":{"@github/spark":"*"}}"#, &[]).unwrap();
        assert!(!s.is_spark);
    }

    #[test]
    fn scan_bare_llm_call_and_kv_in_one_file_lists_file_in_both() {
        let s = scan("{}", &[("src/hooks.ts", "const [x] = useKV('x', 0); const r = await llm(p);")]).unwrap();
        assert_eq!(s.llm_calls, vec!["src/hooks.ts"]);
        assert_eq!(s.kv_uses, vec!["src/hooks.ts"]);
    }

    #[test]
    fn scan_many_sources_keeps_input_order() {
        let s = scan(
            "{}",
            &[("src/z.ts", "spark.llm(a)"), ("src/a.ts", "nothing here"), ("src/m.ts", "llm(b)")],
        )
        .unwrap();
        assert_eq!(s.llm_calls, vec!["src/z.ts", "src/m.ts"]);
        assert!(s.kv_uses.is_empty());
    }

    #[test]
    fn scan_invalid_package_json_returns_error() {
        assert!(scan("{ not json", &[("src/a.ts", "llm(x)")]).unwrap_err().is_syntax());
    }

    #[test]
    fn scan_non_object_package_json_is_not_spark() {
        assert_eq!(scan("[]", &[]).unwrap(), SparkScan::default());
    }
}
