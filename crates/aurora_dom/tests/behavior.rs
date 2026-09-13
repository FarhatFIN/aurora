//! `aurora_dom` behavior tests (§5.6): tree building, mutation, clone,
//! tombstones, serialization. Hand-written per the WHATWG DOM standard's
//! algorithms; the WPT `dom/` corpus arrives with M10 (§8.6).

// §9.6: tests are exempt from the unwrap/expect restrictions.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use aurora_dom::{Document, DomError, serialize};

/// Builds html > head > title, body > p.a > [text, b > text].
#[test]
fn builds_a_tree_and_serializes() {
    let mut document = Document::new();
    let html = document.create_element("html");
    let head = document.create_element("head");
    let title = document.create_element("title");
    let title_text = document.create_text("Hello");
    let body = document.create_element("body");
    let paragraph = document.create_element("p");
    document.set_attribute(paragraph, "class", "a").unwrap();
    let text = document.create_text("hi & <b>not-bold</b>");
    let bold = document.create_element("b");
    let bold_text = document.create_text("bold");
    document.append(document.root(), html).unwrap();
    document.append(html, head).unwrap();
    document.append(head, title).unwrap();
    document.append(title, title_text).unwrap();
    document.append(html, body).unwrap();
    document.append(body, paragraph).unwrap();
    document.append(paragraph, text).unwrap();
    document.append(paragraph, bold).unwrap();
    document.append(bold, bold_text).unwrap();

    assert_eq!(
        serialize(&document, document.root()),
        "<html><head><title>Hello</title></head><body><p class=\"a\">hi &amp; &lt;b&gt;not-bold&lt;/b&gt;<b>bold</b></p></body></html>"
    );
}

#[test]
fn serialization_round_trips_attributes_and_comments() {
    let mut document = Document::new();
    let div = document.create_element("div");
    document.append(document.root(), div).unwrap();
    let comment = document.create_comment(" a <comment> & more ");
    document.append(div, comment).unwrap();
    let img = document.create_element("img");
    document.set_attribute(img, "src", "a&b\"c").unwrap();
    document.set_attribute(img, "alt", "nbsp:\u{00A0}").unwrap();
    document.append(div, img).unwrap();

    assert_eq!(
        serialize(&document, document.root()),
        "<div><!-- a <comment> & more --><img src=\"a&amp;b&quot;c\" alt=\"nbsp:&nbsp;\"></div>"
    );
}

#[test]
fn rawtext_and_rcdata_elements_serialize_their_text() {
    let mut document = Document::new();
    let script = document.create_element("script");
    document.append(document.root(), script).unwrap();
    let code = document.create_text("if (a < b && c > d) { x = \"&amp;\"; }");
    document.append(script, code).unwrap();
    let title = document.create_element("title");
    document.append(document.root(), title).unwrap();
    let heading = document.create_text("a & <b> — 5 > 3");
    document.append(title, heading).unwrap();

    let serialized = serialize(&document, document.root());
    assert!(serialized.contains("<script>if (a < b && c > d) { x = \"&amp;\"; }</script>"));
    assert!(serialized.contains("<title>a &amp; &lt;b&gt; — 5 &gt; 3</title>"));
}

#[test]
fn insert_before_orders_siblings() {
    let mut document = Document::new();
    let parent = document.create_element("ul");
    document.append(document.root(), parent).unwrap();
    let one = document.create_element("li");
    let three = document.create_element("li");
    document.append(parent, one).unwrap();
    document.append(parent, three).unwrap();
    let two = document.create_element("li");
    document.insert_before(parent, two, Some(three)).unwrap();
    let children: Vec<_> = document.children(parent).collect();
    assert_eq!(children, vec![one, two, three]);
    // `None` reference appends.
    let four = document.create_element("li");
    document.insert_before(parent, four, None).unwrap();
    assert_eq!(document.children(parent).count(), 4);
}

#[test]
fn remove_detaches_and_reinsertion_works() {
    let mut document = Document::new();
    let parent = document.create_element("div");
    document.append(document.root(), parent).unwrap();
    let child = document.create_text("x");
    document.append(parent, child).unwrap();
    document.remove(child).unwrap();
    assert!(document.node(child).unwrap().parent().is_none());
    assert_eq!(document.children(parent).count(), 0);
    // A detached node can be re-attached.
    document.append(parent, child).unwrap();
    assert_eq!(document.children(parent).count(), 1);
}

#[test]
fn contains_and_cycle_prevention() {
    let mut document = Document::new();
    let outer = document.create_element("div");
    let inner = document.create_element("div");
    document.append(document.root(), outer).unwrap();
    document.append(outer, inner).unwrap();
    assert!(document.contains(outer, inner));
    assert!(document.contains(outer, outer));
    assert!(!document.contains(inner, outer));
    // Nesting a node inside its own descendant is rejected.
    assert_eq!(
        document.append(inner, outer),
        Err(DomError::InvalidOperation)
    );
}

#[test]
fn fragment_and_doctype_serialize() {
    let mut document = Document::new();
    let fragment = document.create_fragment();
    document.append(document.root(), fragment).unwrap();
    let doctype = document.create_doctype("html", None, None);
    document.append(fragment, doctype).unwrap();

    assert_eq!(serialize(&document, fragment), "<!DOCTYPE html>");
}

#[test]
fn deep_nesting_serializes_without_recursing_to_overflow() {
    let mut document = Document::new();
    let mut parent = document.create_element("div");
    document.append(document.root(), parent).unwrap();
    for _ in 0..50_000 {
        let child = document.create_element("i");
        document.append(parent, child).unwrap();
        parent = child;
    }
    let serialized = serialize(&document, document.root());
    assert_eq!(serialized.matches("<i>").count(), 50_000);
    assert!(serialized.ends_with("</i></div>"));
}
