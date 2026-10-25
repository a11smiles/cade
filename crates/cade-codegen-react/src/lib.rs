//! React + design-system code generation for App projects.
//!
//! The App workspace keeps design and code in sync both ways. This module
//! owns the design → code direction for a screen: it emits TSX that imports
//! the project's real design-system components (never look-alikes). The
//! reverse direction (parse TSX edits back onto the canvas) is APP backlog.

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

/// A name from the design that can't be written into the generated file.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CodegenError {
    #[error("\"{0}\" isn't a valid component name")]
    InvalidComponent(String),
    #[error("\"{0}\" isn't a valid prop name")]
    InvalidProp(String),
    #[error("\"{0}\" isn't a valid screen name; use a capitalized identifier")]
    InvalidScreenName(String),
    #[error("\"{0}\" isn't a valid package name")]
    InvalidPackage(String),
}

/// A JavaScript identifier (ASCII only): `[A-Za-z_$][A-Za-z0-9_$]*`.
fn is_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

/// A JSX element name: a dotted member path (`Form.Row`) or a lowercase
/// custom element (`my-element`).
fn is_component_name(s: &str) -> bool {
    if s.contains('-') {
        s.starts_with(|c: char| c.is_ascii_lowercase())
            && s.split('-')
                .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()))
    } else {
        s.split('.').all(is_identifier)
    }
}

/// A JSX attribute name: an identifier that may also contain `-` (`aria-label`).
fn is_prop_name(s: &str) -> bool {
    let mut chars = s.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$' || c == '-')
}

/// Every component and prop name in the tree can be written into JSX as is.
fn check_names(e: &Element) -> Result<(), CodegenError> {
    if !is_component_name(&e.component) {
        return Err(CodegenError::InvalidComponent(e.component.clone()));
    }
    if let Some((k, _)) = e.props.iter().find(|(k, _)| !is_prop_name(k)) {
        return Err(CodegenError::InvalidProp(k.clone()));
    }
    e.children.iter().try_for_each(check_names)
}

/// Emit a screen component that imports from `package`. Names are validated,
/// not escaped: a name that isn't a valid identifier is design data that can't
/// become code, so it's rejected rather than rewritten.
pub fn screen(name: &str, package: &str, root: &Element) -> Result<String, CodegenError> {
    if !(is_identifier(name) && name.starts_with(|c: char| c.is_ascii_uppercase())) {
        return Err(CodegenError::InvalidScreenName(name.into()));
    }
    if package.is_empty() || package.chars().any(|c| c == '\'' || c == '\\' || c.is_control()) {
        return Err(CodegenError::InvalidPackage(package.into()));
    }
    check_names(root)?;
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
    #[test]
    fn emits_ds_imports_and_jsx() {
        let tree = Element::new("Stepper.Page")
            .prop("step", "{1}")
            .prop("title", "What happened?")
            .child(Element::new("DateField").prop("label", "Date of incident"))
            .child(Element::new("Button").prop("variant", "primary").text("Continue"));
        let tsx = screen("IncidentStep", "@contoso/ds", &tree).unwrap();
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
            screen("IncidentStep", "@contoso/ds", &tree).unwrap(),
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
            screen("Empty", "@contoso/ds", &Element::new("Spacer")).unwrap(),
            "import { Spacer } from '@contoso/ds';\n\nexport function Empty() {\n  return (\n    <Spacer />\n  );\n}\n"
        );
    }

    #[test]
    fn screen_lowercase_html_elements_are_not_imported() {
        let tree = Element::new("div")
            .child(Element::new("Button").text("Go"))
            .child(Element::new("span"));
        assert!(
            screen("S", "@contoso/ds", &tree)
                .unwrap()
                .starts_with("import { Button } from '@contoso/ds';\n")
        );
    }

    #[test]
    fn screen_repeated_and_dotted_components_import_each_root_once() {
        let tree = Element::new("Form")
            .child(Element::new("Form.Row").child(Element::new("TextField")))
            .child(Element::new("Form.Row").child(Element::new("TextField")));
        assert!(
            screen("S", "@contoso/ds", &tree)
                .unwrap()
                .starts_with("import { Form, TextField } from '@contoso/ds';\n")
        );
    }

    #[test]
    fn screen_prop_with_quote_is_emitted_as_escaped_string_expression() {
        let tsx = screen("S", "@contoso/ds", &Element::new("Button").prop("title", "Say \"hi\"")).unwrap();
        assert!(tsx.contains("    <Button title={\"Say \\\"hi\\\"\"} />\n"), "{tsx}");
    }

    #[test]
    fn screen_text_with_braces_and_angle_is_emitted_as_string_expression() {
        let tsx = screen("S", "@contoso/ds", &Element::new("Button").text("a < b {x}")).unwrap();
        assert!(tsx.contains("    <Button>\n      {\"a < b {x}\"}\n    </Button>\n"), "{tsx}");
    }

    #[test]
    fn screen_prop_with_backslash_and_newline_escapes_them() {
        let tsx = screen("S", "@contoso/ds", &Element::new("Field").prop("hint", "C:\\path\nnext")).unwrap();
        assert!(tsx.contains("    <Field hint={\"C:\\\\path\\nnext\"} />\n"), "{tsx}");
    }

    #[test]
    fn screen_prop_brace_value_that_is_not_a_literal_is_emitted_as_string() {
        let tsx = screen(
            "S",
            "@contoso/ds",
            &Element::new("Button").prop("label", "{alert(document.cookie)}"),
        )
        .unwrap();
        assert!(tsx.contains("    <Button label={\"{alert(document.cookie)}\"} />\n"), "{tsx}");
    }

    #[test]
    fn screen_prop_and_text_with_ampersand_are_emitted_as_string_expressions() {
        let tsx = screen(
            "S",
            "@contoso/ds",
            &Element::new("Tag").prop("title", "Tom &amp; Jerry").text("R&D"),
        )
        .unwrap();
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
        let tsx = screen("S", "@contoso/ds", &tree).unwrap();
        assert!(
            tsx.contains("    <Stepper step={1} ratio={-0.5} open={true} busy={false} value={null} />\n"),
            "{tsx}"
        );
    }

    #[test]
    fn screen_component_name_with_injected_attribute_returns_invalid_component() {
        for name in [
            "Button onClick={steal}",
            "Button/",
            "Form..Row",
            ".Row",
            "Row.",
            "",
            "1Button",
            "my--el",
            "Button>",
        ] {
            assert_eq!(
                screen("S", "@contoso/ds", &Element::new("Card").child(Element::new(name))),
                Err(CodegenError::InvalidComponent(name.into())),
                "{name:?}"
            );
        }
    }

    #[test]
    fn screen_prop_name_with_injected_attribute_returns_invalid_prop() {
        for name in ["onClick={steal} title", "title=", "a b", "", "-x", "x\"", "{...spread}"] {
            assert_eq!(
                screen("S", "@contoso/ds", &Element::new("Button").prop(name, "x")),
                Err(CodegenError::InvalidProp(name.into())),
                "{name:?}"
            );
        }
    }

    #[test]
    fn screen_valid_component_and_prop_names_are_accepted() {
        let tree = Element::new("div")
            .prop("data-step", "1")
            .prop("aria-label", "Steps")
            .prop("$ref", "x")
            .child(Element::new("Form.Row").prop("onChange", "x").prop("_private", "x"))
            .child(Element::new("my-element"))
            .child(Element::new("$Icon"));
        let tsx = screen("Steps", "@contoso/ds", &tree).unwrap();
        assert!(tsx.contains("    <div data-step=\"1\" aria-label=\"Steps\" $ref=\"x\">\n"), "{tsx}");
        assert!(tsx.contains("      <Form.Row onChange=\"x\" _private=\"x\" />\n"), "{tsx}");
        assert!(tsx.contains("      <my-element />\n"), "{tsx}");
    }

    #[test]
    fn screen_screen_name_not_a_capitalized_identifier_returns_invalid_screen_name() {
        for name in ["steps", "Steps() {}; function X", "Step-1", "", "class"] {
            assert_eq!(
                screen(name, "@contoso/ds", &Element::new("Button")),
                Err(CodegenError::InvalidScreenName(name.into())),
                "{name:?}"
            );
        }
    }

    #[test]
    fn screen_package_with_quote_or_newline_returns_invalid_package() {
        for package in ["@contoso/ds'; import evil from 'x", "a\\b", "a\nb", ""] {
            assert_eq!(
                screen("S", package, &Element::new("Button")),
                Err(CodegenError::InvalidPackage(package.into())),
                "{package:?}"
            );
        }
    }

    #[test]
    fn screen_text_and_children_puts_text_before_children() {
        let tree = Element::new("Callout").text("Heads up").child(Element::new("Link").text("Details"));
        let tsx = screen("S", "@contoso/ds", &tree).unwrap();
        assert!(tsx.contains("    <Callout>\n      Heads up\n      <Link>\n        Details\n      </Link>\n    </Callout>\n"));
    }
}
