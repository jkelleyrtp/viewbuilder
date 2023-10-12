use crate::{
    element::{TextElement, ViewElement},
    tree::Tree,
};
use dioxus::{
    core::{ElementId, Mutation},
    prelude::{Component, TemplateNode, VirtualDom},
};
use skia_safe::{Font, Typeface};
use slotmap::DefaultKey;
use std::collections::HashMap;

enum Node {
    Text(String),
    Element { children: Vec<Self> },
}

impl Node {
    fn from_template(template_node: &TemplateNode) -> Self {
        match template_node {
            TemplateNode::Text { text } => Node::Text(text.to_string()),
            TemplateNode::Element {
                tag: _,
                namespace: _,
                attrs: _,
                children,
            } => {
                let children = children.into_iter().map(Self::from_template).collect();
                Node::Element { children }
            }
            _ => todo!(),
        }
    }
}

struct Template {
    roots: Vec<Node>,
}

pub struct VirtualTree {
    pub(crate) vdom: VirtualDom,
    pub(crate) tree: Tree,
    templates: HashMap<String, Template>,
    elements: HashMap<ElementId, DefaultKey>,
    pub(crate) root: DefaultKey,
}

impl VirtualTree {
    pub fn new(app: Component) -> Self {
        let mut tree = Tree::default();
        let root = tree.insert(Box::new(ViewElement::new(Vec::new())));

        let mut elements = HashMap::new();
        elements.insert(ElementId(0), root);

        Self {
            vdom: VirtualDom::new(app),
            tree,
            templates: HashMap::new(),
            elements,
            root,
        }
    }

    pub fn rebuild(&mut self) {
        let mutations = self.vdom.rebuild();
        dbg!(&mutations);
        for template in mutations.templates {
            let roots = template.roots.iter().map(Node::from_template).collect();
            self.templates
                .insert(template.name.to_string(), Template { roots });
        }

        let mut stack = Vec::new();
        for edit in mutations.edits {
            match edit {
                Mutation::LoadTemplate { name, index, id } => {
                    let root = &self.templates[name].roots[index];
                    let key = insert(&mut self.tree, root);
                    self.elements.insert(id, key);
                    stack.push(key);
                }
                Mutation::AppendChildren { id, m } => {
                    let key = self.elements[&id];
                    let elem = self.tree.get_mut(key);
                    for _ in 0..m {
                        let child_key = stack.pop().unwrap();
                        elem.push_child(child_key);
                    }
                }
                _ => todo!(),
            }
        }
    }
}

fn insert(tree: &mut Tree, node: &Node) -> DefaultKey {
    match node {
        Node::Text(text) => {
            let typeface = Typeface::new("Arial", Default::default()).unwrap();
            let font = Font::new(typeface, 100.);

            tree.insert(Box::new(TextElement::new(&text, &font)))
        }
        Node::Element { children } => {
            let child_keys = children
                .into_iter()
                .map(|child| insert(tree, child))
                .collect();
            tree.insert(Box::new(ViewElement::new(child_keys)))
        }
    }
}
