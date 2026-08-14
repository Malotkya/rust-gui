use std::fmt;
use std::ops::{Deref, DerefMut};

mod element;
pub use element::*;
mod document;
pub use document::*;
mod inner;
pub(crate) use inner::*;
mod item;
pub use item::*;
mod list;
pub use list::*;

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum NodeError {
    NotDesendent(NodeItem, NodeItem),
    CannotAppendToTextNode,
    CannotSetAttributeOfTextNode,
    NodeRefIsInvalid
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotDesendent(parrent, child) 
                => write!(f, "Child NodeItem is not a desendent!\nParrent: {}\nChild: {}", parrent, child),
            Self::CannotAppendToTextNode
                => write!(f, "Unable to append child to TextNodeItem!"),
            Self::CannotSetAttributeOfTextNode
                => write!(f, "Unable to set attribute of TextNodeItem!"),
            Self::NodeRefIsInvalid
                => write!(f, "Node has been dropped, and NodeRef has been invalidated!")
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum NodeType {
    Element,
    Text,
    Document,
    Fragment
}

pub trait Node {
    fn node_type(&self) -> NodeType;
    fn tag_name(&self) -> &str;

    fn child_nodes(&self) -> &NodeList<NodeItem>;
    //fn child_nodes_mut(&mut self) -> &mut NodeList;

    fn get_content(&self) -> String;
    fn set_content<T:ToString>(&mut self, content:T);

    fn parrent_node(&self) -> Option<NodeItem>;
    fn contains<T:PartialEq<NodeItem>>(&self, node:&T) -> bool;

    fn append_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: DerefMut<Target = NodeItem>;
    fn prepend_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: DerefMut<Target = NodeItem>;
    fn insert_before<N, R>(&mut self, new_node:&mut N, ref_node:&R) -> Result<(), NodeError>
        where N: DerefMut<Target = NodeItem>,
              R: Deref<Target = NodeItem>;

    fn remove_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: DerefMut<Target = NodeItem>;

    fn node(&self) -> NodeItem;
}

impl<T:DerefMut<Target = NodeItem>> Node for T {
    #[inline]
    fn node_type(&self) -> NodeType {
        self.deref().node_type()
    }

    #[inline]
    fn tag_name(&self) -> &str {
        self.deref().tag_name()
    }

    #[inline]
    fn child_nodes(&self) -> &NodeList<NodeItem> {
        self.deref()
            .child_nodes()
    }

    #[inline]
    fn get_content(&self) -> String {
        self.deref().get_content()
    }

    #[inline]
    fn set_content<S:ToString>(&mut self, content:S) {
        self.deref_mut()
            .set_content(content);
    }

    #[inline]
    fn parrent_node(&self) -> Option<NodeItem> {
        self.deref().parrent_node()
    }

    #[inline]
    fn contains<N:PartialEq<NodeItem>>(&self, node:&N) -> bool {
        self.deref().contains(node)
    }

    #[inline]
    fn append_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: DerefMut<Target = NodeItem>
    {
        self.deref_mut().append_node(node)
    }

    #[inline]
    fn prepend_node<N>(&mut self, node:&mut N) -> Result<(), NodeError> 
        where N: DerefMut<Target = NodeItem>
    {
        self.deref_mut().prepend_node(node)
    }

    #[inline]
    fn insert_before<N, R>(&mut self, new_node:&mut N, ref_node:&R) -> Result<(), NodeError>
        where N: DerefMut<Target = NodeItem>,
              R: Deref<Target = NodeItem>
    {
        self.deref_mut()
            .insert_before(new_node, ref_node)
    }

    #[inline]
    fn remove_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: DerefMut<Target = NodeItem>
    {
        self.deref_mut().remove_node(node)
    }

    #[inline]
    fn node(&self) -> NodeItem {
        self.deref().node()
    }
}