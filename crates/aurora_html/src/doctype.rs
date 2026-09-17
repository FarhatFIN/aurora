use crate::token::Token;
use aurora_dom::{Document, DomError, Node, NodeData, NodeId, QuirksMode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Doctype {
    pub name: String,
    pub public_id: Option<String>,
    pub system_id: Option<String>,
    pub force_quirks: bool,
}

impl Doctype {
    #[must_use]
    pub fn from_token(token: &Token) -> Option<Self> {
        let Token::Doctype {
            name,
            public,
            system,
            force_quirks,
        } = token
        else {
            return None;
        };
        Some(Self {
            name: name.clone().unwrap_or_default(),
            public_id: public.clone(),
            system_id: system.clone(),
            force_quirks: *force_quirks,
        })
    }

    #[must_use]
    pub fn quirks(&self) -> QuirksMode {
        if self.quirks_trigger() {
            return QuirksMode::Quirks;
        }
        if self.limited_quirks_trigger() {
            return QuirksMode::LimitedQuirks;
        }
        QuirksMode::NoQuirks
    }

    fn starts_with_ascii_ignore_case(prefix: &str, value: Option<&str>) -> bool {
        value.is_some_and(|v| {
            v.len() >= prefix.len()
                && v.as_bytes()
                    .iter()
                    .zip(prefix.as_bytes())
                    .all(|(b, p)| b.to_ascii_lowercase() == *p)
        })
    }

    const QUIRKS_PUBLIC_IDENTIFIERS: &[&str] = &[
        "+//silmaril//dtd html pro v0r11 19970101//",
        "-//as//dtd html 3.0 aswedit + extensions//",
        "-//advasoft ltd//dtd html 3.0 aswedit + extensions//",
        "-//ietf//dtd html 2.0 level 1//",
        "-//ietf//dtd html 2.0 level 2//",
        "-//ietf//dtd html 2.0 strict level 1//",
        "-//ietf//dtd html 2.0 strict level 2//",
        "-//ietf//dtd html 2.0 strict//",
        "-//ietf//dtd html 2.0//",
        "-//ietf//dtd html 2.1e//",
        "-//ietf//dtd html 3.0//",
        "-//ietf//dtd html 3.2 final//",
        "-//ietf//dtd html 3.2//",
        "-//ietf//dtd html 3//",
        "-//ietf//dtd html level 0//",
        "-//ietf//dtd html level 1//",
        "-//ietf//dtd html level 2//",
        "-//ietf//dtd html level 3//",
        "-//ietf//dtd html strict level 0//",
        "-//ietf//dtd html strict level 1//",
        "-//ietf//dtd html strict level 2//",
        "-//ietf//dtd html strict level 3//",
        "-//ietf//dtd html strict//",
        "-//ietf//dtd html//",
        "-//metrius//dtd metrius presentational//",
        "-//microsoft//dtd internet explorer 2.0 html strict//",
        "-//microsoft//dtd internet explorer 2.0 html//",
        "-//microsoft//dtd internet explorer 2.0 tables//",
        "-//microsoft//dtd internet explorer 3.0 html strict//",
        "-//microsoft//dtd internet explorer 3.0 html//",
        "-//microsoft//dtd internet explorer 3.0 tables//",
        "-//netscape comm. corp.//dtd html//",
        "-//netscape comm. corp.//dtd strict html//",
        "-//o'reilly and associates//dtd html 2.0//",
        "-//o'reilly and associates//dtd html extended 1.0//",
        "-//o'reilly and associates//dtd html extended relaxed 1.0//",
        "-//sq//dtd html 2.0 hotmetal + extensions//",
        "-//softquad software//dtd hotmetal pro 6.0::19990601::extensions to html 4.0//",
        "-//softquad//dtd hotmetal pro 4.0::19971010::extensions to html 4.0//",
        "-//spyglass//dtd html 2.0 extended//",
        "-//sun microsystems corp.//dtd hotjava html//",
        "-//sun microsystems corp.//dtd hotjava strict html//",
        "-//w3c//dtd html 3 1995-03-24//",
        "-//w3c//dtd html 3.2 draft//",
        "-//w3c//dtd html 3.2 final//",
        "-//w3c//dtd html 3.2//",
        "-//w3c//dtd html 3.2s draft//",
        "-//w3c//dtd html 4.0 frameset//",
        "-//w3c//dtd html 4.0 transitional//",
        "-//w3c//dtd html experimental 19960712//",
        "-//w3c//dtd html experimental 970421//",
        "-//w3c//dtd w3 html//",
        "-//w3o//dtd w3 html 3.0//",
        "-//webtechs//dtd mozilla html 2.0//",
        "-//webtechs//dtd mozilla html//",
    ];
    const QUIRKS_EXACT_PUBLIC_IDENTIFIERS: &[&str] = &[
        "-//w3o//dtd w3 html strict 3.0//en//",
        "-/w3c/dtd html 4.0 transitional/en",
        "html",
    ];
    const QUIRKS_SYSTEM_IDENTIFIER: &str =
        "http://www.ibm.com/data/dtd/v11/ibmxhtml1-transitional.dtd";
    const LIMITED_QUIRKS_PUBLIC_PREFIXES: &[&str] = &[
        "-//w3c//dtd xhtml 1.0 frameset//",
        "-//w3c//dtd xhtml 1.0 transitional//",
        "-//w3c//dtd html 4.01 frameset//",
        "-//w3c//dtd html 4.01 transitional//",
    ];

    fn quirks_trigger(&self) -> bool {
        if self.force_quirks {
            return true;
        }
        if !self.name.eq_ignore_ascii_case("html") {
            return true;
        }
        if self
            .system_id
            .as_deref()
            .is_some_and(|system| system.eq_ignore_ascii_case(Self::QUIRKS_SYSTEM_IDENTIFIER))
        {
            return true;
        }
        let Some(public) = &self.public_id else {
            return false;
        };
        if Self::QUIRKS_EXACT_PUBLIC_IDENTIFIERS
            .iter()
            .any(|candidate| public.eq_ignore_ascii_case(candidate))
        {
            return true;
        }
        if Self::QUIRKS_PUBLIC_IDENTIFIERS
            .iter()
            .any(|prefix| Self::starts_with_ascii_ignore_case(prefix, Some(public)))
        {
            return true;
        }
        if Self::LIMITED_QUIRKS_PUBLIC_PREFIXES[2..]
            .iter()
            .any(|prefix| Self::starts_with_ascii_ignore_case(prefix, Some(public)))
        {
            return self.system_id.as_ref().is_none_or(String::is_empty);
        }
        false
    }

    fn limited_quirks_trigger(&self) -> bool {
        Self::LIMITED_QUIRKS_PUBLIC_PREFIXES
            .iter()
            .any(|prefix| Self::starts_with_ascii_ignore_case(prefix, self.public_id.as_deref()))
            && self
                .system_id
                .as_ref()
                .is_none_or(|system| !system.is_empty())
            || Self::LIMITED_QUIRKS_PUBLIC_PREFIXES[0..2]
                .iter()
                .any(|prefix| {
                    Self::starts_with_ascii_ignore_case(prefix, self.public_id.as_deref())
                })
    }

    ///
    /// # Errors
    /// [`aurora_dom::DomError`] from the underlying tree mutation.
    pub fn insert_into_initial_document(
        &self,
        document: &mut Document,
    ) -> Result<Option<NodeId>, DomError> {
        let root = document.root();
        let preamble = document.children(root).find(|child| {
            matches!(
                document.node(*child).map(Node::data),
                Some(NodeData::DocumentType { .. } | NodeData::Element(_))
            )
        });
        if preamble.is_some() {
            return Ok(None);
        }
        let node = document.create_doctype(
            &self.name,
            Some(self.public_id.as_deref().unwrap_or_default()),
            Some(self.system_id.as_deref().unwrap_or_default()),
        );
        document.append(root, node)?;
        document.set_quirks(self.quirks());
        Ok(Some(node))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn doctype(name: &str, public: Option<&str>, system: Option<&str>) -> Doctype {
        Doctype {
            name: name.to_owned(),
            public_id: public.map(str::to_owned),
            system_id: system.map(str::to_owned),
            force_quirks: false,
        }
    }

    #[test]
    fn html_without_identifiers_is_no_quirks() {
        assert_eq!(doctype("html", None, None).quirks(), QuirksMode::NoQuirks);
        assert_eq!(
            doctype("HTML", None, Some("about:legacy-compat")).quirks(),
            QuirksMode::NoQuirks
        );
    }

    #[test]
    fn missing_or_non_html_name_is_quirks() {
        assert_eq!(doctype("", None, None).quirks(), QuirksMode::Quirks);
        assert_eq!(doctype("svg", None, None).quirks(), QuirksMode::Quirks);
    }

    #[test]
    fn html401_transitional_split_by_system_identifier() {
        let public = "-//W3C//DTD HTML 4.01 Transitional//EN";
        assert_eq!(
            doctype("html", Some(public), None).quirks(),
            QuirksMode::Quirks
        );
        assert_eq!(
            doctype("html", Some(public), Some("")).quirks(),
            QuirksMode::Quirks
        );
        assert_eq!(
            doctype("html", Some(public), Some("x.dtd")).quirks(),
            QuirksMode::LimitedQuirks
        );
        let parsed_missing = Doctype::from_token(&Token::Doctype {
            name: Some("html".into()),
            public: Some(public.into()),
            system: None,
            force_quirks: false,
        })
        .unwrap();
        let parsed_empty = Doctype::from_token(&Token::Doctype {
            name: Some("html".into()),
            public: Some(public.into()),
            system: Some(String::new()),
            force_quirks: false,
        })
        .unwrap();
        assert_ne!(parsed_missing.system_id, parsed_empty.system_id);
        assert_eq!(parsed_missing.quirks(), parsed_empty.quirks());
    }

    #[test]
    fn xhtml10_transitional_is_limited_quirks() {
        assert_eq!(
            doctype(
                "html",
                Some("-//W3C//DTD XHTML 1.0 Transitional//EN"),
                Some("x.dtd")
            )
            .quirks(),
            QuirksMode::LimitedQuirks
        );
        assert_eq!(
            doctype("html", Some("-//W3C//DTD XHTML 1.0 Frameset//EN"), None).quirks(),
            QuirksMode::LimitedQuirks
        );
    }

    #[test]
    fn ibm_system_identifier_matches_case_insensitively() {
        let system = "HTTP://WWW.IBM.COM/data/dtd/v11/ibmxhtml1-transitional.DTD";
        assert_eq!(
            doctype("html", None, Some(system)).quirks(),
            QuirksMode::Quirks
        );
        assert_eq!(
            doctype("html", None, Some("http://www.ibm.com/other.dtd")).quirks(),
            QuirksMode::NoQuirks
        );
    }

    #[test]
    fn every_quirks_public_prefix_matches_and_near_misses_do_not() {
        for prefix in Doctype::QUIRKS_PUBLIC_IDENTIFIERS {
            assert_eq!(
                doctype("html", Some(prefix), None).quirks(),
                QuirksMode::Quirks,
                "{prefix:?}"
            );
            let uppercased = prefix.to_ascii_uppercase();
            assert_eq!(
                doctype("html", Some(&uppercased), None).quirks(),
                QuirksMode::Quirks,
                "{prefix:?}"
            );
        }
        for near_miss in [
            "-//IETF//DTD HTML 2.0/EN",
            "-//W3C//DTD HTML 3.2 Fina//",
            "-//W3C//DTD XHTML 1.0 Strict//EN",
            "-//W3C//DTD HTML 4.01//EN",
        ] {
            assert_eq!(
                doctype("html", Some(near_miss), None).quirks(),
                QuirksMode::NoQuirks,
                "{near_miss:?}"
            );
        }
    }

    #[test]
    fn every_limited_quirks_public_prefix_matches() {
        for prefix in Doctype::LIMITED_QUIRKS_PUBLIC_PREFIXES {
            assert_eq!(
                doctype("html", Some(prefix), Some("x.dtd")).quirks(),
                QuirksMode::LimitedQuirks,
                "{prefix:?}"
            );
            let uppercased = prefix.to_ascii_uppercase();
            assert_eq!(
                doctype("html", Some(&uppercased), Some("x.dtd")).quirks(),
                QuirksMode::LimitedQuirks,
                "{prefix:?}"
            );
        }
    }

    #[test]
    fn forced_doctype_is_quirks_and_still_stores() {
        let forced = Doctype {
            force_quirks: true,
            ..doctype("html", None, None)
        };
        assert_eq!(forced.quirks(), QuirksMode::Quirks);
        let mut document = Document::new();
        assert!(matches!(
            forced.insert_into_initial_document(&mut document),
            Ok(Some(_))
        ));
        assert_eq!(document.quirks(), QuirksMode::Quirks);
    }

    #[test]
    fn duplicate_doctype_and_element_block_insertion_without_mutation() {
        let no_quirks = Doctype {
            name: "html".into(),
            public_id: None,
            system_id: None,
            force_quirks: false,
        };
        let mut document = Document::new();
        let first = no_quirks
            .insert_into_initial_document(&mut document)
            .unwrap()
            .unwrap();
        assert_eq!(document.quirks(), QuirksMode::NoQuirks);
        let quirky = Doctype {
            force_quirks: true,
            ..no_quirks.clone()
        };
        assert_eq!(quirky.insert_into_initial_document(&mut document), Ok(None));
        assert_eq!(document.quirks(), QuirksMode::NoQuirks);
        assert_eq!(
            document.children(document.root()).collect::<Vec<_>>(),
            [first]
        );
        let element = document.create_element("html");
        document.append(document.root(), element).unwrap();
        assert_eq!(quirky.insert_into_initial_document(&mut document), Ok(None));
        assert_eq!(document.quirks(), QuirksMode::NoQuirks);
        assert_eq!(
            document.children(document.root()).collect::<Vec<_>>(),
            [first, element]
        );
        assert_eq!(
            document.node(first).unwrap().parent(),
            Some(document.root())
        );
    }

    #[test]
    fn from_token_keeps_force_quirks_and_other_tokens_rejected() {
        assert_eq!(Doctype::from_token(&Token::Character("x".into())), None);
        let forced = Token::Doctype {
            name: Some("html".into()),
            public: None,
            system: None,
            force_quirks: true,
        };
        let parsed = Doctype::from_token(&forced).unwrap();
        assert!(parsed.force_quirks);
        assert_eq!(parsed.quirks(), QuirksMode::Quirks);
        let normal = Token::Doctype {
            name: Some("html".into()),
            public: None,
            system: Some("about:legacy-compat".into()),
            force_quirks: false,
        };
        assert_eq!(
            Doctype::from_token(&normal),
            Some(doctype("html", None, Some("about:legacy-compat")))
        );
    }

    #[test]
    fn from_token_uses_empty_string_for_a_missing_name() {
        let nameless = Token::Doctype {
            name: None,
            public: Some("HTML".into()),
            system: None,
            force_quirks: false,
        };
        let parsed = Doctype::from_token(&nameless).unwrap();
        assert_eq!(parsed.name, "");
        assert_eq!(parsed.quirks(), QuirksMode::Quirks);
    }
}
