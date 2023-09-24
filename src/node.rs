use crate::element::ElementData;
use accesskit::NodeBuilder;
use skia_safe::{Canvas, Color4f, Font, FontStyle, Paint, Rect, TextBlob, Typeface};
use slotmap::DefaultKey;
use std::borrow::Cow;
use taffy::{
    prelude::{Layout, Size},
    style::Style,
    Taffy,
};

/// Kind of data type of a node.
#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    /// Element node kind.
    Element,

    /// Text node kind.
    Text,
}

/// Data type of a node.
pub enum NodeData {
    /// Element node.
    Element(ElementData),

    /// Text node.
    Text(Cow<'static, str>),
}

pub struct Node {
    /// Data type of the node.
    pub data: NodeData,

    /// Parent node id.
    pub parent: Option<DefaultKey>,

    /// Child node ids.
    pub children: Option<Vec<DefaultKey>>,

    /// Layout key for the taffy node.
    pub layout_key: Option<DefaultKey>,

    /// Absolute layout of the node, relative to the window.
    pub layout: Option<Layout>,
}

impl Node {
    /// Create a new node from its data.
    pub fn new(data: NodeData) -> Self {
        Self {
            data,
            parent: None,
            children: None,
            layout_key: None,
            layout: None,
        }
    }

    /// Create a new text node.
    pub fn text(content: impl Into<Cow<'static, str>>) -> Self {
        Self::new(NodeData::Text(content.into()))
    }

    /// Get the node kind.
    pub fn kind(&self) -> NodeKind {
        match self.data {
            NodeData::Element { .. } => NodeKind::Element,
            NodeData::Text(_) => NodeKind::Text,
        }
    }

    /// Build a semantics node.
    pub fn semantics(&self) -> NodeBuilder {
        NodeBuilder::default()
    }

    /// Setup the layout node.
    pub fn layout(&mut self, taffy: &mut Taffy) {
        let mut style = Style::default();
        match self.data {
            NodeData::Element(ref mut elem) => {
                if let Some(size) = elem.size {
                    style.size = size;
                }

                if let Some(flex_direction) = elem.flex_direction {
                    style.flex_direction = flex_direction;
                }

                style.align_items = elem.align_items;
                style.justify_content = elem.justify_content;
            }
            _ => {}
        }

        if let Some(layout_key) = self.layout_key {
            taffy.set_style(layout_key, style).unwrap();
        } else {
            let layout_key = taffy.new_leaf(style).unwrap();
            self.layout_key = Some(layout_key);
        }

        if let NodeData::Text(ref content) = self.data {
            let typeface = Typeface::new("Arial", FontStyle::default()).unwrap();
            let font = Font::new(typeface, 100.);
            let text_blob = TextBlob::new(content, &font).unwrap();
            let bounds = text_blob.bounds().clone();

            // TODO this is a measure func for paragraphs
            taffy
                .set_measure(
                    self.layout_key.unwrap(),
                    Some(taffy::node::MeasureFunc::Boxed(Box::new(move |_, _| {
                        Size {
                            width: bounds.width() / 2.,
                            height: bounds.height() / 2.,
                        }
                    }))),
                )
                .unwrap();
        }
    }

    /// Paint the node to a skia canvas.
    pub fn paint(&mut self, canvas: &mut Canvas) {
        let layout = self.layout.as_ref().unwrap();
        match &self.data {
            NodeData::Element(elem) => {
                if let Some(background_color) = elem.background_color {
                    let paint = Paint::new(background_color, None);
                    canvas.draw_rect(
                        Rect::new(
                            layout.location.x,
                            layout.location.y,
                            layout.location.x + layout.size.width,
                            layout.location.y + layout.size.height,
                        ),
                        &paint,
                    );
                }
            }
            NodeData::Text(content) => {
                let typeface = Typeface::new("Arial", FontStyle::default()).unwrap();
                let font = Font::new(typeface, 100.);
                let text_blob = TextBlob::new(content, &font).unwrap();
                let paint = Paint::new(Color4f::new(0., 0., 0., 1.), None);
                let height = text_blob.bounds().height();
                canvas.draw_text_blob(
                    text_blob,
                    (layout.location.x, layout.location.y + height / 2.),
                    &paint,
                );
            }
        }
    }
}

impl From<&'static str> for Node {
    fn from(value: &'static str) -> Self {
        Self::text(value)
    }
}
