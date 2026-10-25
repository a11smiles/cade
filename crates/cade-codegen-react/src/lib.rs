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

fn render(e: &Element, depth: usize, out: &mut String) {
    let pad = "  ".repeat(depth);
    let props: String = e
        .props
        .iter()
        .map(|(k, v)| {
            if v.starts_with('{') {
                format!(" {k}={v}")
            } else {
                format!(" {k}=\"{v}\"")
            }
        })
        .collect();
    if e.children.is_empty() && e.text.is_none() {
        out.push_str(&format!("{pad}<{}{props} />\n", e.component));
        return;
    }
    out.push_str(&format!("{pad}<{}{props}>\n", e.component));
    if let Some(t) = &e.text {
        out.push_str(&format!("{pad}  {t}\n"));
    }
    for c in &e.children {
        render(c, depth + 1, out);
    }
    out.push_str(&format!("{pad}</{}>\n", e.component));
}

/// Emit a screen component that imports from `package`.
pub fn screen(name: &str, package: &str, root: &Element) -> String {
    let mut imports = vec![];
    collect(root, &mut imports);
    imports.sort();
    let mut body = String::new();
    render(root, 2, &mut body);
    format!(
        "import {{ {} }} from '{package}';\n\nexport function {name}() {{\n  return (\n{body}  );\n}}\n",
        imports.join(", ")
    )
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
        let tsx = screen("IncidentStep", "@contoso/ds", &tree);
        assert!(tsx.starts_with("import { Button, DateField, Stepper } from '@contoso/ds';"));
        assert!(tsx.contains("<Stepper.Page step={1} title=\"What happened?\">"));
        assert!(tsx.contains("<DateField label=\"Date of incident\" />"));
    }
}
