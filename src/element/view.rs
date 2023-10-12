use super::Element;
use crate::layout::Layout;
use slotmap::DefaultKey;

pub struct ViewElement {
    children: Vec<DefaultKey>,
}

impl ViewElement {
    pub fn new(children: Vec<DefaultKey>) -> Self {
        Self { children }
    }
}

impl Element for ViewElement {
    fn children(&mut self) -> Option<Vec<slotmap::DefaultKey>> {
        Some(self.children.clone())
    }

    fn layout(&mut self) -> crate::layout::Builder {
        Layout::builder()
    }

    fn semantics(&mut self) -> accesskit::NodeBuilder {
        todo!()
    }

    fn paint(&mut self, _layout: &crate::layout::Layout, _canvas: &mut skia_safe::Canvas) {}
}
