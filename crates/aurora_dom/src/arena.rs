//! The node arena (§4.5): arena-allocated nodes with stable generational
//! indices, logical removal via tombstones, physical reclamation at
//! document teardown. Stale handles are detectable values, not UB.

/// A node handle: arena index in the low 32 bits, generation in the high
/// 32 bits. A removed node's slot bumps its generation, so a `NodeId`
/// copied before the removal stops resolving (§4.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub(crate) u64);

impl NodeId {
    pub(crate) fn new(index: usize, generation: u32) -> Self {
        Self((u64::from(generation) << 32) | u64::try_from(index).unwrap_or(u64::MAX))
    }

    pub(crate) fn index(self) -> usize {
        (self.0 & 0xFFFF_FFFF) as usize
    }

    pub(crate) fn generation(self) -> u32 {
        (self.0 >> 32) as u32
    }
}

/// The payload of a node (§5.6's `NodeData`, the M2 subset: script-visible
/// shadow roots and attribute nodes arrive with M10/M12).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeData {
    /// The document node; every arena has exactly one.
    Document,
    /// `<!DOCTYPE name PUBLIC "public_id" "system_id">`.
    DocumentType {
        name: String,
        public_id: Option<String>,
        system_id: Option<String>,
    },
    /// An element with its attribute list in insertion order (§5.6
    /// pitfall: attribute order matters for serialization).
    Element(ElementData),
    /// Character data.
    Text(String),
    /// `<!-- contents -->`.
    Comment(String),
    /// `<?target data?>` (serialization keeps it for XML round-trips).
    ProcessingInstruction { target: String, data: String },
    /// A detached subtree container (the future `template` contents host).
    DocumentFragment,
}

/// An element's name and attributes (§5.6's `ElementData`, HTML-only for
/// M2 — the namespace model with xmlns fixups lands with the SVG/MathML
/// insertion modes, tracked in PROGRESS.md).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ElementData {
    /// The local name, lowercase for HTML elements.
    pub name: String,
    /// Attributes in insertion order; duplicates are a tree-constructor
    /// concern (it deduplicates per the HTML spec's duplicate-attribute
    /// parse error).
    pub attributes: Vec<(String, String)>,
}

impl ElementData {
    /// The first attribute value for `name`, ASCII-case-insensitive.
    #[must_use]
    pub fn attribute(&self, name: &str) -> Option<&str> {
        let lowered = name.to_ascii_lowercase();
        self.attributes
            .iter()
            .find(|(n, _)| n == &lowered)
            .map(|(_, v)| v.as_str())
    }

    /// Sets (replacing) or adds an attribute, preserving insertion order.
    pub fn set_attribute(&mut self, name: &str, value: &str) {
        let lowered = name.to_ascii_lowercase();
        for (n, v) in &mut self.attributes {
            if n == &lowered {
                value.clone_into(v);
                return;
            }
        }
        self.attributes.push((lowered, value.to_owned()));
    }
}

/// A node in the arena: tree links plus payload (§5.6's `Node`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub(crate) id: NodeId,
    pub(crate) parent: Option<NodeId>,
    pub(crate) first_child: Option<NodeId>,
    pub(crate) last_child: Option<NodeId>,
    pub(crate) prev_sibling: Option<NodeId>,
    pub(crate) next_sibling: Option<NodeId>,
    pub(crate) data: NodeData,
}

impl Node {
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.id
    }

    #[must_use]
    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    #[must_use]
    pub fn first_child(&self) -> Option<NodeId> {
        self.first_child
    }

    #[must_use]
    pub fn last_child(&self) -> Option<NodeId> {
        self.last_child
    }

    #[must_use]
    pub fn next_sibling(&self) -> Option<NodeId> {
        self.next_sibling
    }

    #[must_use]
    pub fn prev_sibling(&self) -> Option<NodeId> {
        self.prev_sibling
    }

    #[must_use]
    pub fn data(&self) -> &NodeData {
        &self.data
    }

    #[must_use]
    pub fn is_element(&self) -> bool {
        matches!(self.data, NodeData::Element(_))
    }
}

/// One arena slot: the node, or a tombstone with the bumped generation.
#[derive(Clone, Debug)]
pub(crate) struct Slot {
    pub(crate) generation: u32,
    pub(crate) node: Option<Node>,
}
