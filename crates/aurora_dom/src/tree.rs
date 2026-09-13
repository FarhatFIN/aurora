//! The `Document`: the arena plus every tree mutation (§5.6 invariant:
//! *all* mutation goes through `Document` methods so observers and
//! invalidation hooks cannot be bypassed). Operations are total — a stale
//! `NodeId` is a typed [`DomError::StaleNode`], never a panic (§4.6).

use crate::arena::{ElementData, Node, NodeData, NodeId, Slot};

/// DOM operation failures (§4.6 class 1: values, not exceptions).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DomError {
    /// The handle predates a removal or never existed in this document.
    StaleNode,
    /// The operation's precondition about the tree shape did not hold.
    InvalidOperation,
}

/// Quirks mode, decided by the tree constructor from the DOCTYPE (§5.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuirksMode {
    NoQuirks,
    LimitedQuirks,
    Quirks,
}

/// The document: arena, root handle, and the mutation surface.
#[derive(Clone, Debug)]
pub struct Document {
    pub(crate) arena: Vec<Slot>,
    pub(crate) root: NodeId,
    pub(crate) quirks: QuirksMode,
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl Document {
    /// A new document with just the document node.
    #[must_use]
    pub fn new() -> Self {
        let node = Node {
            id: NodeId::new(0, 0),
            parent: None,
            first_child: None,
            last_child: None,
            prev_sibling: None,
            next_sibling: None,
            data: NodeData::Document,
        };
        Self {
            arena: vec![Slot {
                generation: 0,
                node: Some(node),
            }],
            root: NodeId::new(0, 0),
            quirks: QuirksMode::NoQuirks,
        }
    }

    #[must_use]
    pub fn root(&self) -> NodeId {
        self.root
    }

    #[must_use]
    pub fn quirks(&self) -> QuirksMode {
        self.quirks
    }

    /// Resolves a handle to its node, if live (generation check, §4.5).
    #[must_use]
    pub fn node(&self, id: NodeId) -> Option<&Node> {
        let slot = self.arena.get(id.index())?;
        if slot.generation == id.generation() {
            slot.node.as_ref()
        } else {
            None
        }
    }

    fn node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        let slot = self.arena.get_mut(id.index())?;
        if slot.generation == id.generation() {
            slot.node.as_mut()
        } else {
            None
        }
    }

    fn live(&self, id: NodeId) -> Result<(), DomError> {
        if self.node(id).is_some() {
            Ok(())
        } else {
            Err(DomError::StaleNode)
        }
    }

    // ---- node creation ---------------------------------------------------

    fn allocate(&mut self, data: NodeData) -> NodeId {
        let id = NodeId::new(self.arena.len(), 0);
        self.arena.push(Slot {
            generation: 0,
            node: Some(Node {
                id,
                parent: None,
                first_child: None,
                last_child: None,
                prev_sibling: None,
                next_sibling: None,
                data,
            }),
        });
        id
    }

    /// Creates an element (HTML-only name space in M2, lowercase name).
    #[must_use]
    pub fn create_element(&mut self, name: &str) -> NodeId {
        self.allocate(NodeData::Element(ElementData {
            name: name.to_ascii_lowercase(),
            attributes: Vec::new(),
        }))
    }

    /// Creates a text node.
    #[must_use]
    pub fn create_text(&mut self, data: &str) -> NodeId {
        self.allocate(NodeData::Text(data.to_owned()))
    }

    /// Creates a comment node.
    #[must_use]
    pub fn create_comment(&mut self, data: &str) -> NodeId {
        self.allocate(NodeData::Comment(data.to_owned()))
    }

    /// Creates a doctype node.
    #[must_use]
    pub fn create_doctype(
        &mut self,
        name: &str,
        public_id: Option<&str>,
        system_id: Option<&str>,
    ) -> NodeId {
        self.allocate(NodeData::DocumentType {
            name: name.to_owned(),
            public_id: public_id.map(str::to_owned),
            system_id: system_id.map(str::to_owned),
        })
    }

    /// Creates a document fragment.
    #[must_use]
    pub fn create_fragment(&mut self) -> NodeId {
        self.allocate(NodeData::DocumentFragment)
    }

    // ---- mutation --------------------------------------------------------

    /// Appends `child` as the last child of `parent`. The child must be
    /// detached (the tree constructor detaches before reparenting).
    ///
    /// # Errors
    /// [`DomError::StaleNode`] for dead handles; [`DomError::InvalidOperation`]
    /// when `child` still has a parent or would be nested inside itself.
    pub fn append(&mut self, parent: NodeId, child: NodeId) -> Result<(), DomError> {
        self.live(parent)?;
        self.live(child)?;
        if self.node(child).and_then(|n| n.parent).is_some() {
            return Err(DomError::InvalidOperation);
        }
        // Prevent cycles: a node cannot contain its own ancestor chain.
        let mut cursor = Some(parent);
        while let Some(ancestor) = cursor {
            if ancestor == child {
                return Err(DomError::InvalidOperation);
            }
            cursor = self.node(ancestor).and_then(|n| n.parent);
        }
        self.detach_from_parent(child);
        let last = self.node(parent).and_then(|n| n.last_child);
        {
            let child_node = self.node_mut(child).ok_or(DomError::StaleNode)?;
            child_node.parent = Some(parent);
            child_node.prev_sibling = last;
            child_node.next_sibling = None;
        }
        if let Some(last_id) = last
            && let Some(last_node) = self.node_mut(last_id)
        {
            last_node.next_sibling = Some(child);
        }
        let parent_node = self.node_mut(parent).ok_or(DomError::StaleNode)?;
        if parent_node.first_child.is_none() {
            parent_node.first_child = Some(child);
        }
        parent_node.last_child = Some(child);
        Ok(())
    }

    /// Inserts `new` immediately before `reference` among `parent`'s
    /// children; `reference` must be a child of `parent`. Appends when
    /// `reference` is `None`.
    ///
    /// # Errors
    /// [`DomError::StaleNode`] / [`DomError::InvalidOperation`] per
    /// [`Document::append`], plus a non-child `reference`.
    pub fn insert_before(
        &mut self,
        parent: NodeId,
        new: NodeId,
        reference: Option<NodeId>,
    ) -> Result<(), DomError> {
        let Some(reference) = reference else {
            return self.append(parent, new);
        };
        self.live(parent)?;
        self.live(new)?;
        self.live(reference)?;
        if self.node(reference).and_then(|n| n.parent) != Some(parent) {
            return Err(DomError::InvalidOperation);
        }
        if self.node(new).and_then(|n| n.parent).is_some() {
            return Err(DomError::InvalidOperation);
        }
        let prev = self.node(reference).and_then(|n| n.prev_sibling);
        self.detach_from_parent(new);
        {
            let new_node = self.node_mut(new).ok_or(DomError::StaleNode)?;
            new_node.parent = Some(parent);
            new_node.prev_sibling = prev;
            new_node.next_sibling = Some(reference);
        }
        if let Some(prev_id) = prev
            && let Some(prev_node) = self.node_mut(prev_id)
        {
            prev_node.next_sibling = Some(new);
        }
        let reference_node = self.node_mut(reference).ok_or(DomError::StaleNode)?;
        reference_node.prev_sibling = Some(new);
        if self.node(parent).and_then(|n| n.first_child) == Some(reference)
            && let Some(parent_node) = self.node_mut(parent)
        {
            parent_node.first_child = Some(new);
        }
        Ok(())
    }

    /// Detaches `child` from its parent, if any.
    ///
    /// # Errors
    /// [`DomError::StaleNode`] for dead handles.
    pub fn remove(&mut self, child: NodeId) -> Result<(), DomError> {
        self.live(child)?;
        self.detach_from_parent(child);
        Ok(())
    }

    /// Logical removal: tombstone the slot and bump its generation so every
    /// outstanding handle for the old node stops resolving (§4.5).
    pub(crate) fn tombstone(&mut self, id: NodeId) {
        let slot = &mut self.arena[id.index()];
        slot.node = None;
        slot.generation = slot.generation.wrapping_add(1);
    }

    fn detach_from_parent(&mut self, child: NodeId) {
        let (parent, prev, next) = match self.node(child) {
            Some(node) => (node.parent, node.prev_sibling, node.next_sibling),
            None => return,
        };
        if let Some(prev_id) = prev
            && let Some(prev_node) = self.node_mut(prev_id)
        {
            prev_node.next_sibling = next;
        }
        if let Some(next_id) = next
            && let Some(next_node) = self.node_mut(next_id)
        {
            next_node.prev_sibling = prev;
        }
        if let Some(parent_id) = parent
            && let Some(parent_node) = self.node_mut(parent_id)
        {
            if parent_node.first_child == Some(child) {
                parent_node.first_child = next;
            }
            if parent_node.last_child == Some(child) {
                parent_node.last_child = prev;
            }
        }
        if let Some(child_node) = self.node_mut(child) {
            child_node.parent = None;
            child_node.prev_sibling = None;
            child_node.next_sibling = None;
        }
    }

    /// Sets (replacing) an attribute on an element (§5.6: attributes mutate
    /// only through the document).
    ///
    /// # Errors
    /// [`DomError::StaleNode`] for a dead handle; [`DomError::InvalidOperation`]
    /// when the node is not an element.
    pub fn set_attribute(&mut self, id: NodeId, name: &str, value: &str) -> Result<(), DomError> {
        self.live(id)?;
        let node = self.node_mut(id).ok_or(DomError::StaleNode)?;
        match &mut node.data {
            NodeData::Element(element) => {
                element.set_attribute(name, value);
                Ok(())
            }
            _ => Err(DomError::InvalidOperation),
        }
    }

    // ---- queries ---------------------------------------------------------

    /// Whether `maybe_ancestor` is `node` itself or contains it.
    #[must_use]
    pub fn contains(&self, maybe_ancestor: NodeId, node: NodeId) -> bool {
        if maybe_ancestor == node {
            return true;
        }
        let mut cursor = self.node(node).and_then(|n| n.parent);
        while let Some(current) = cursor {
            if current == maybe_ancestor {
                return true;
            }
            cursor = self.node(current).and_then(|n| n.parent);
        }
        false
    }

    /// The element name, if this node is an element.
    #[must_use]
    pub fn element_name(&self, id: NodeId) -> Option<&str> {
        match self.node(id).map(Node::data) {
            Some(NodeData::Element(element)) => Some(&element.name),
            _ => None,
        }
    }

    /// Children of `id`, first to last. Dead handles yield no items.
    #[must_use]
    pub fn children(&self, id: NodeId) -> Children<'_> {
        Children {
            document: self,
            next: self.node(id).and_then(|n| n.first_child),
        }
    }

    /// The last (deepest) descendant: the tree constructor's "appropriate
    /// place to insert" helper.
    #[must_use]
    pub fn last_descendant(&self, id: NodeId) -> NodeId {
        let mut current = id;
        while let Some(last) = self.node(current).and_then(|n| n.last_child) {
            current = last;
        }
        current
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        // The document-lifetime decision (§4.5): every live slot is
        // tombstoned so no handle outlives the arena's ownership.
        for index in 0..self.arena.len() {
            if self.arena[index].node.is_some() {
                let generation = self.arena[index].generation;
                self.tombstone(NodeId::new(index, generation));
            }
        }
    }
}

/// Iterator over a node's children.
#[derive(Clone, Debug)]
pub struct Children<'a> {
    document: &'a Document,
    next: Option<NodeId>,
}

impl Iterator for Children<'_> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        let id = self.next?;
        self.next = self.document.node(id).and_then(|n| n.next_sibling);
        Some(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tombstoned_handles_stop_resolving() {
        // Tombstoning applies to detached nodes (the tree constructor
        // discards parse-time temporaries this way; teardown tombstones all).
        let mut document = Document::new();
        let parent = document.create_element("div");
        document.append(document.root(), parent).unwrap();
        let child = document.create_text("x");
        document.append(parent, child).unwrap();
        document.remove(child).unwrap();
        assert!(document.node(child).is_some());
        document.tombstone(child);
        // The slot's generation bumped: the old handle is dead.
        assert_eq!(document.node(child), None);
        assert_eq!(document.append(parent, child), Err(DomError::StaleNode));
    }
}
