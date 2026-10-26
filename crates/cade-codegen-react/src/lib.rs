//! React + design-system code generation for App projects.
//!
//! The App workspace keeps design and code in sync both ways. This module
//! owns the design → code direction for a screen: it emits TSX that imports
//! the project's real design-system components (never look-alikes). The
//! reverse direction (parse TSX edits back onto the canvas) is APP backlog.

/// Why a screen could not be generated.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CodegenError {
    #[error("invalid component name: {0:?}")]
    InvalidComponentName(String),
    #[error("invalid prop name: {0:?}")]
    InvalidPropName(String),
    #[error("invalid screen name: {0:?}")]
    InvalidScreenName(String),
    #[error("invalid package name: {0:?}")]
    InvalidPackageName(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub component: String,
    pub props: Vec<(String, String)>,
    pub children: Vec<Element>,
    pub text: Option<String>,
}

impl Element {
    pub fn new(component: &str) -> Self {
        Element {
            component: component.into(),
            props: vec![],
            children: vec![],
            text: None,
        }
    }
    pub fn prop(mut self, k: &str, v: &str) -> Self {
        self.props.push((k.into(), v.into()));
        self
    }
    pub fn text(mut self, t: &str) -> Self {
        self.text = Some(t.into());
        self
    }
    pub fn child(mut self, c: Element) -> Self {
        self.children.push(c);
        self
    }
}

fn collect(e: &Element, out: &mut Vec<String>) {
    let root = e.component.split('.').next().unwrap_or(&e.component).to_string();
    if root.chars().next().is_some_and(|c| c.is_uppercase()) && !out.contains(&root) {
        out.push(root);
    }
    for c in &e.children {
        collect(c, out);
    }
}

/// A JavaScript double-quoted string literal for `s`.
fn js_string(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{2028}' | '\u{2029}' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// True when `s` can sit inside JSX quotes or as JSX text without any
/// character being read as markup, an entity or an expression.
fn jsx_safe(s: &str) -> bool {
    !s.chars()
        .any(|c| matches!(c, '"' | '\\' | '{' | '}' | '<' | '>' | '&') || c.is_control())
}

/// A `{…}` prop value passes through as an expression only when it's a
/// number, boolean or null literal; anything else is design data, not code.
fn literal_expression(v: &str) -> Option<&str> {
    let inner = v.strip_prefix('{')?.strip_suffix('}')?.trim();
    let literal = matches!(inner, "true" | "false" | "null") || (!inner.is_empty() && inner.parse::<f64>().is_ok_and(f64::is_finite));
    literal.then_some(v)
}

fn is_identifier(s: &str, allow_hyphen: bool) -> bool {
    let mut chars = s.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$' || (allow_hyphen && c == '-'))
}

fn valid_component(name: &str) -> bool {
    name.split('.').all(|part| is_identifier(part, false))
}

fn valid_package(name: &str) -> bool {
    !name.is_empty()
        && !name
            .chars()
            .any(|c| matches!(c, '\'' | '\\' | '`') || c.is_control() || c.is_whitespace())
}

fn validate(e: &Element) -> Result<(), CodegenError> {
    if !valid_component(&e.component) {
        return Err(CodegenError::InvalidComponentName(e.component.clone()));
    }
    if let Some((k, _)) = e.props.iter().find(|(k, _)| !is_identifier(k, true)) {
        return Err(CodegenError::InvalidPropName(k.clone()));
    }
    e.children.iter().try_for_each(validate)
}

fn prop(k: &str, v: &str) -> String {
    if let Some(expr) = literal_expression(v) {
        format!(" {k}={expr}")
    } else if jsx_safe(v) {
        format!(" {k}=\"{v}\"")
    } else {
        format!(" {k}={{{}}}", js_string(v))
    }
}

fn text(t: &str) -> String {
    if jsx_safe(t) {
        t.to_string()
    } else {
        format!("{{{}}}", js_string(t))
    }
}

fn render(e: &Element, depth: usize, out: &mut String) {
    let pad = "  ".repeat(depth);
    let props: String = e.props.iter().map(|(k, v)| prop(k, v)).collect();
    if e.children.is_empty() && e.text.is_none() {
        out.push_str(&format!("{pad}<{}{props} />\n", e.component));
        return;
    }
    out.push_str(&format!("{pad}<{}{props}>\n", e.component));
    if let Some(t) = &e.text {
        out.push_str(&format!("{pad}  {}\n", text(t)));
    }
    for c in &e.children {
        render(c, depth + 1, out);
    }
    out.push_str(&format!("{pad}</{}>\n", e.component));
}

/// Emit a screen component that imports from `package`.
pub fn screen(name: &str, package: &str, root: &Element) -> Result<String, CodegenError> {
    if !is_identifier(name, false) {
        return Err(CodegenError::InvalidScreenName(name.into()));
    }
    if !valid_package(package) {
        return Err(CodegenError::InvalidPackageName(package.into()));
    }
    validate(root)?;
    let mut imports = vec![];
    collect(root, &mut imports);
    imports.sort();
    let mut body = String::new();
    render(root, 2, &mut body);
    Ok(format!(
        "import {{ {} }} from '{package}';\n\nexport function {name}() {{\n  return (\n{body}  );\n}}\n",
        imports.join(", ")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn screen(name: &str, package: &str, root: &Element) -> String {
        super::screen(name, package, root).unwrap()
    }

    #[test]
    fn screen_rejects_hostile_prop_names() {
        let tree = Element::new("Button").prop("onClick={steal} title", "x");
        assert_eq!(
            super::screen("S", "@contoso/ds", &tree),
            Err(CodegenError::InvalidPropName("onClick={steal} title".into()))
        );
    }

    #[test]
    fn screen_rejects_hostile_component_names_including_nested() {
        for name in ["Button onClick={steal}", "", "A..B", "Button>", "1Btn"] {
            let tree = Element::new("Form").child(Element::new(name));
            assert_eq!(
                super::screen("S", "@contoso/ds", &tree),
                Err(CodegenError::InvalidComponentName(name.into()))
            );
        }
    }

    #[test]
    fn screen_rejects_hostile_screen_and_package_names() {
        let tree = Element::new("Button");
        assert!(matches!(
            super::screen("S() {}; evil", "@contoso/ds", &tree),
            Err(CodegenError::InvalidScreenName(_))
        ));
        assert!(matches!(
            super::screen("S", "x';evil('", &tree),
            Err(CodegenError::InvalidPackageName(_))
        ));
    }

    #[test]
    fn screen_accepts_dotted_data_and_aria_names() {
        let tree = Element::new("Menu.Item").prop("data-x", "1").prop("aria-label", "Open");
        let tsx = screen("S", "@contoso/ds", &tree);
        assert!(tsx.contains("<Menu.Item data-x=\"1\" aria-label=\"Open\" />"));
    }

    #[test]
    fn emits_ds_imports_and_jsx() {
        let tree = Element::new("Stepper.Page")
            .prop("step", "{1}")
            .prop("title", "What happened?")
            .child(Element::new("DateField").prop("label", "Date of incident"))
            .child(Element::new("Button").prop("variant", "primary").text("Continue"));
        let tsx = screen("IncidentStep", "@contoso/ds", &tree);
        assert!(tsx.starts_with("import { Button, DateField, Stepper } from '@contoso/ds';"));
        assert!(tsx.contains("<Stepper.Page step={1} title=\"What happened?\">"));
        assert!(tsx.contains("<DateField label=\"Date of incident\" />"));
    }

    #[test]
    fn element_new_builders_set_props_text_and_children_in_order() {
        let e = Element::new("Card")
            .prop("tone", "info")
            .prop("count", "{3}")
            .text("Hello")
            .child(Element::new("Badge"))
            .child(Element::new("Button"));
        assert_eq!(
            e,
            Element {
                component: "Card".into(),
                props: vec![("tone".into(), "info".into()), ("count".into(), "{3}".into())],
                children: vec![Element::new("Badge"), Element::new("Button")],
                text: Some("Hello".into()),
            }
        );
    }

    #[test]
    fn screen_nested_tree_emits_exact_tsx() {
        let tree = Element::new("Stepper.Page")
            .prop("step", "{1}")
            .prop("title", "What happened?")
            .child(Element::new("DateField").prop("label", "Date of incident"))
            .child(Element::new("Button").prop("variant", "primary").text("Continue"));
        assert_eq!(
            screen("IncidentStep", "@contoso/ds", &tree),
            "import { Button, DateField, Stepper } from '@contoso/ds';\n\
             \n\
             export function IncidentStep() {\n\
             \x20 return (\n\
             \x20   <Stepper.Page step={1} title=\"What happened?\">\n\
             \x20     <DateField label=\"Date of incident\" />\n\
             \x20     <Button variant=\"primary\">\n\
             \x20       Continue\n\
             \x20     </Button>\n\
             \x20   </Stepper.Page>\n\
             \x20 );\n\
             }\n"
        );
    }

    #[test]
    fn screen_single_empty_element_emits_self_closing_tag() {
        assert_eq!(
            screen("Empty", "@contoso/ds", &Element::new("Spacer")),
            "import { Spacer } from '@contoso/ds';\n\nexport function Empty() {\n  return (\n    <Spacer />\n  );\n}\n"
        );
    }

    #[test]
    fn screen_lowercase_html_elements_are_not_imported() {
        let tree = Element::new("div")
            .child(Element::new("Button").text("Go"))
            .child(Element::new("span"));
        assert!(screen("S", "@contoso/ds", &tree).starts_with("import { Button } from '@contoso/ds';\n"));
    }

    #[test]
    fn screen_repeated_and_dotted_components_import_each_root_once() {
        let tree = Element::new("Form")
            .child(Element::new("Form.Row").child(Element::new("TextField")))
            .child(Element::new("Form.Row").child(Element::new("TextField")));
        assert!(screen("S", "@contoso/ds", &tree).starts_with("import { Form, TextField } from '@contoso/ds';\n"));
    }

    #[test]
    fn screen_prop_with_quote_is_emitted_as_escaped_string_expression() {
        let tsx = screen("S", "@contoso/ds", &Element::new("Button").prop("title", "Say \"hi\""));
        assert!(tsx.contains("    <Button title={\"Say \\\"hi\\\"\"} />\n"), "{tsx}");
    }

    #[test]
    fn screen_text_with_braces_and_angle_is_emitted_as_string_expression() {
        let tsx = screen("S", "@contoso/ds", &Element::new("Button").text("a < b {x}"));
        assert!(tsx.contains("    <Button>\n      {\"a < b {x}\"}\n    </Button>\n"), "{tsx}");
    }

    #[test]
    fn screen_prop_with_backslash_and_newline_escapes_them() {
        let tsx = screen("S", "@contoso/ds", &Element::new("Field").prop("hint", "C:\\path\nnext"));
        assert!(tsx.contains("    <Field hint={\"C:\\\\path\\nnext\"} />\n"), "{tsx}");
    }

    #[test]
    fn screen_prop_brace_value_that_is_not_a_literal_is_emitted_as_string() {
        let tsx = screen(
            "S",
            "@contoso/ds",
            &Element::new("Button").prop("label", "{alert(document.cookie)}"),
        );
        assert!(tsx.contains("    <Button label={\"{alert(document.cookie)}\"} />\n"), "{tsx}");
    }

    #[test]
    fn screen_prop_and_text_with_ampersand_are_emitted_as_string_expressions() {
        let tsx = screen(
            "S",
            "@contoso/ds",
            &Element::new("Tag").prop("title", "Tom &amp; Jerry").text("R&D"),
        );
        assert!(
            tsx.contains("    <Tag title={\"Tom &amp; Jerry\"}>\n      {\"R&D\"}\n    </Tag>\n"),
            "{tsx}"
        );
    }

    #[test]
    fn screen_literal_brace_values_stay_expressions() {
        let tree = Element::new("Stepper")
            .prop("step", "{1}")
            .prop("ratio", "{-0.5}")
            .prop("open", "{true}")
            .prop("busy", "{false}")
            .prop("value", "{null}");
        let tsx = screen("S", "@contoso/ds", &tree);
        assert!(
            tsx.contains("    <Stepper step={1} ratio={-0.5} open={true} busy={false} value={null} />\n"),
            "{tsx}"
        );
    }

    #[test]
    fn screen_text_and_children_puts_text_before_children() {
        let tree = Element::new("Callout").text("Heads up").child(Element::new("Link").text("Details"));
        let tsx = screen("S", "@contoso/ds", &tree);
        assert!(tsx.contains("    <Callout>\n      Heads up\n      <Link>\n        Details\n      </Link>\n    </Callout>\n"));
    }
}
