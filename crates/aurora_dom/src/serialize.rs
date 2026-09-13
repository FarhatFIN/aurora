//! HTML serialization (WHATWG DOM Parsing and Serialization, "HTML
//! fragment serialization algorithm") — the M2 surface: elements,
//! attributes, text, comments, doctype, processing instructions.

use crate::arena::{Node, NodeData, NodeId};
use crate::tree::Document;

/// Elements that serialize no end tag (WHATWG HTML §13.1.2 "void elements").
const VOID_ELEMENTS: [&str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

/// Elements whose text children serialize literally (raw text).
const RAWTEXT_ELEMENTS: [&str; 7] = [
    "style",
    "script",
    "xmp",
    "iframe",
    "noembed",
    "noframes",
    "plaintext",
];

/// Serializes `id` (and its subtree) to an HTML string.
#[must_use]
pub fn serialize(document: &Document, id: NodeId) -> String {
    let mut out = String::new();
    // Explicit stack, not recursion: hostile input can nest deeply and a
    // stack overflow would violate the no-panic rule (§4.6). The bool marks
    // "exiting" (all children emitted; emit the end tag).
    let mut stack = vec![(id, false)];
    while let Some((current, exiting)) = stack.pop() {
        let Some(node) = document.node(current) else {
            continue; // stale handle inside the tree: skip, never panic
        };
        match node.data() {
            NodeData::Document | NodeData::DocumentFragment => {
                if !exiting {
                    push_children_reversed(document, &mut stack, node);
                }
            }
            NodeData::DocumentType {
                name,
                public_id,
                system_id,
            } => {
                serialize_doctype(&mut out, name, public_id.as_deref(), system_id.as_deref());
            }
            NodeData::Element(element) => {
                if exiting {
                    if !VOID_ELEMENTS.contains(&element.name.as_str()) {
                        out.push_str("</");
                        out.push_str(&element.name);
                        out.push('>');
                    }
                    continue;
                }
                out.push('<');
                out.push_str(&element.name);
                for (name, value) in &element.attributes {
                    out.push(' ');
                    out.push_str(name);
                    out.push_str("=\"");
                    escape(&mut out, value, EscapeSet::Attribute);
                    out.push('"');
                }
                out.push('>');
                if VOID_ELEMENTS.contains(&element.name.as_str()) {
                    continue;
                }
                // The exit marker goes on first so every child (and every
                // descendant) pops before the end tag is emitted.
                stack.push((current, true));
                push_children_reversed(document, &mut stack, node);
            }
            NodeData::Text(data) => {
                // Raw-text parents serialize literally; everything else —
                // including RCDATA parents — uses the standard text escape
                // set (&, U+00A0, <, >; the serializer has no separate
                // RCDATA set — RCDATA affects parsing, not serialization).
                let set = match parent_element_name(document, node) {
                    Some(name) if RAWTEXT_ELEMENTS.contains(&name) => EscapeSet::Literal,
                    _ => EscapeSet::Text,
                };
                escape(&mut out, data, set);
            }
            NodeData::Comment(data) => {
                out.push_str("<!--");
                out.push_str(data);
                out.push_str("-->");
            }
            NodeData::ProcessingInstruction { target, data } => {
                out.push_str("<?");
                out.push_str(target);
                out.push(' ');
                out.push_str(data);
                out.push_str("?>");
            }
        }
    }
    out
}

fn parent_element_name<'a>(document: &'a Document, node: &Node) -> Option<&'a str> {
    document.element_name(node.parent()?)
}

/// Pushes a node's children so they pop in document order.
fn push_children_reversed(document: &Document, stack: &mut Vec<(NodeId, bool)>, node: &Node) {
    let mut children = Vec::new();
    let mut cursor = node.first_child();
    while let Some(child) = cursor {
        children.push(child);
        cursor = document.node(child).and_then(|n| n.next_sibling);
    }
    for child in children.into_iter().rev() {
        stack.push((child, false));
    }
}

fn serialize_doctype(
    out: &mut String,
    name: &str,
    public_id: Option<&str>,
    system_id: Option<&str>,
) {
    out.push_str("<!DOCTYPE ");
    out.push_str(name);
    if let Some(public) = public_id {
        out.push_str(" PUBLIC \"");
        out.push_str(public);
        out.push('"');
        if let Some(system) = system_id {
            out.push_str(" \"");
            out.push_str(system);
            out.push('"');
        }
    } else if let Some(system) = system_id {
        out.push_str(" SYSTEM \"");
        out.push_str(system);
        out.push('"');
    }
    out.push('>');
}

/// The escape sets (WHATWG "escaping a string": text escapes `&`, U+00A0,
/// `<`, `>`; attribute values escape `&`, U+00A0, `"`).
#[derive(Clone, Copy)]
enum EscapeSet {
    Literal,
    Text,
    Attribute,
}

fn escape(out: &mut String, text: &str, set: EscapeSet) {
    for c in text.chars() {
        match (c, set) {
            ('&', EscapeSet::Literal) => out.push('&'),
            ('&', _) => out.push_str("&amp;"),
            ('\u{00A0}', EscapeSet::Literal) => out.push('\u{00A0}'),
            ('\u{00A0}', _) => out.push_str("&nbsp;"),
            ('<', EscapeSet::Text) => out.push_str("&lt;"),
            ('>', EscapeSet::Text) => out.push_str("&gt;"),
            ('"', EscapeSet::Attribute) => out.push_str("&quot;"),
            (other, _) => out.push(other),
        }
    }
}
