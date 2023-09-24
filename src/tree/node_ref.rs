use crate::{element::ElementData, node::NodeData, Node, Tree};
use skia_safe::Color4f;
use slotmap::DefaultKey;

/// Reference to an element in a tree.
pub struct NodeRef<'a> {
    pub(crate) key: DefaultKey,
    pub(crate) tree: &'a mut Tree,
}

impl<'a> NodeRef<'a> {
    /// Move the reference to the parent element.
    pub fn parent(&mut self) -> &mut Self {
        let parent_key = self.node().parent.unwrap();
        self.key = parent_key;
        self
    }

    /// Get a reference the current node.
    pub fn node(&mut self) -> &mut Node {
        &mut self.tree.nodes.nodes[self.key]
    }

    /// Get a reference the current element.
    ///
    /// ## Panics
    /// This function will panic if the current reference is to a text node,
    /// not to an element.
    pub fn element(&mut self) -> &mut ElementData {
        if let NodeData::Element(ref mut element) = self.node().data {
            element
        } else {
            todo!()
        }
    }

    /// Update the background color.
    pub fn set_background_color(&mut self, color: Color4f) {
        self.as_mut().background_color = Some(color);
        self.tree.inner.changes.push(self.key);
    }
}

impl<'a> AsMut<ElementData> for NodeRef<'a> {
    fn as_mut(&mut self) -> &mut ElementData {
        self.element()
    }
}
