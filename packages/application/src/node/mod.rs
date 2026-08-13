use std::{
    cell::{Ref, RefCell}, ops::{Deref, DerefMut}, rc::{Rc, Weak}
};

pub use element::Attribute;

mod core;
pub use core::*;
mod document;
pub(crate) use document::DocumentData;
mod element;
pub(crate) use element::ElementData;
mod iterator;
pub use iterator::*;
mod text;
pub(crate) use text::TextData;

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct NodeRef(Rc<RefCell<NodeDataType>>);

impl Deref for NodeRef {
    type Target = NodeDataType;

    fn deref(&self) -> &Self::Target {
        unsafe{ &*self.0.as_ptr() }
    }
}

impl DerefMut for NodeRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.as_ptr() }
    }
}

impl Node for NodeRef {
    fn node_type(&self) -> u32 {
        self.deref().node_type()
    }

    fn tag_name<'a>(&'a self) -> &'a str {
        self.deref().tag_name()
    }
    fn children(&self) -> NodeIterator<'_> {
        self.deref().children()
    }

    fn content(&self) -> String {
        self.deref().content()
    }

    fn stringify(&self) -> String {
        self.deref().stringify()
    }

    fn parrent(&self) -> Option<&NodeRef> {
        self.deref().parrent()
    }

    fn contains(&self, other:&impl Node) -> bool {
        self.deref().contains(other)
    }

    fn append(&mut self, value:&impl Node) {
        self.deref_mut().append(value)
    }

    fn remove(&mut self, value:&impl Node) -> Result<(), NodeError> {
        self.deref_mut().remove(value)
    }

    fn children_mut(&mut self) -> NodeIteratorMut<'_> {
        self.deref_mut().children_mut()
    }

    fn set_content<C:ToString>(&mut self, value:C) {
        self.deref_mut().set_content(value)
    }

    fn as_ref(&self) -> NodeRef {
        self.clone()
    }
}

impl NodeMut for NodeRef {
    fn set_parrent(&mut self, value:&impl NodeMut) {
        self.deref_mut().set_parrent(value);
    }

    fn remove_parrent(&mut self) {
        self.deref_mut().remove_parrent();
    }

    fn remove_child(&mut self, child:&impl NodeMut) -> bool {
        self.deref_mut().remove_child(child)
    }
}

impl NodeCmp for NodeRef {}




