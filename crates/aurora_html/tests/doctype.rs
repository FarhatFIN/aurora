#![allow(clippy::unwrap_used, clippy::expect_used)]

use aurora_dom::{Document, NodeData, QuirksMode};
use aurora_html::{Doctype, Tokenizer, TokenizerOptions};

#[test]
fn missing_token_identifiers_become_empty_dom_strings() {
    let mut tokenizer = Tokenizer::new("<!DOCTYPE html>Hello", TokenizerOptions::default());
    let doctype = Doctype::from_token(&tokenizer.next_token().unwrap()).unwrap();
    assert_eq!(doctype.public_id, None);
    assert_eq!(doctype.system_id, None);
    let mut document = Document::new();
    let node = doctype
        .insert_into_initial_document(&mut document)
        .unwrap()
        .unwrap();
    assert_eq!(document.quirks(), QuirksMode::NoQuirks);
    assert_eq!(
        document.node(node).unwrap().data(),
        &NodeData::DocumentType {
            name: "html".into(),
            public_id: Some(String::new()),
            system_id: Some(String::new()),
        }
    );
}

#[test]
fn quirky_tokenizer_doctype_sets_document_mode() {
    let mut tokenizer = Tokenizer::new(
        "<!DOCTYPE html PUBLIC \"+//Silmaril//dtd html Pro v0r11 19970101//EN\">",
        TokenizerOptions::default(),
    );
    let token = tokenizer.next_token().unwrap();
    let doctype = Doctype::from_token(&token).unwrap();
    let mut document = Document::new();
    let node = doctype
        .insert_into_initial_document(&mut document)
        .unwrap()
        .unwrap();
    assert_eq!(document.quirks(), QuirksMode::Quirks);
    assert_eq!(
        document.children(document.root()).collect::<Vec<_>>(),
        [node]
    );
    assert_eq!(
        document.node(node).unwrap().data(),
        &NodeData::DocumentType {
            name: "html".into(),
            public_id: Some("+//Silmaril//dtd html Pro v0r11 19970101//EN".into()),
            system_id: Some(String::new()),
        }
    );
}

#[test]
fn plain_doctype_is_no_quirks_and_legacy_compat_is_kept() {
    let mut document = Document::new();
    let mut count = 0;
    for (input, expected_mode) in [
        ("<!DOCTYPE html>", QuirksMode::NoQuirks),
        (
            "<!DOCTYPE html SYSTEM \"about:legacy-compat\">",
            QuirksMode::NoQuirks,
        ),
    ] {
        let mut tokenizer = Tokenizer::new(input, TokenizerOptions::default());
        let token = tokenizer.next_token().unwrap();
        let doctype = Doctype::from_token(&token).unwrap();
        let inserted = doctype.insert_into_initial_document(&mut document).unwrap();
        count += 1;
        if count == 1 {
            let node = inserted.unwrap();
            assert_eq!(document.quirks(), expected_mode, "{input:?}");
            assert_eq!(
                document.children(document.root()).collect::<Vec<_>>(),
                [node],
                "{input:?}"
            );
        } else {
            assert_eq!(inserted, None, "{input:?}");
            assert_eq!(document.children(document.root()).count(), 1, "{input:?}");
        }
        assert_eq!(document.quirks(), expected_mode, "{input:?}");
    }
}
