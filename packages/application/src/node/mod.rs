use std::fmt;
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
    NodeRefIsInvalid,
    NodeIsNotConnected(NodeItem),
    QueryError(QueryError)
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
                => write!(f, "Node has been dropped, and NodeRef has been invalidated!"),
            Self::NodeIsNotConnected(node)
                => write!(f, "Node is not connected!\n{}", node),
            Self::QueryError(err)
                => fmt::Display::fmt(err, f)
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

pub trait RawNode {
    fn inner(&self) -> NodeItem;
}

impl RawNode for String {
    fn inner(&self) -> NodeItem {
        NodeItem::new_text(self)
    }
}

impl RawNode for &str {
    fn inner(&self) -> NodeItem {
        NodeItem::new_text(self)
    }
}

pub trait Node: RawNode {
    fn node_type(&self) -> NodeType;
    fn tag_name(&self) -> &str;

    fn child_nodes(&self) -> &NodeList<impl Node>;
    //fn child_nodes_mut(&mut self) -> &mut NodeList;

    fn get_content(&self) -> String;
    fn set_content<T:ToString>(&mut self, content:T);

    fn parrent_node(&self) -> impl NodeRef;
    fn contains<T:PartialEq<NodeItem>>(&self, node:&T) -> bool;

    fn append_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: Node;
    fn prepend_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: Node;
    fn insert_before<N, R>(&mut self, new_node:&mut N, ref_node:&R) -> Result<(), NodeError>
        where N: Node,
              R: Node;

    fn insert_after<N, R>(&mut self, new_node:&mut N, ref_node:&R) -> Result<(), NodeError>
        where N: Node, 
              R: Node;

    fn remove_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: Node;

    fn is_connected(&self) -> bool;

}

pub trait NodeRef {
    type TargetNode: RawNode;
    fn to_owned(&self) -> Option<Self::TargetNode>;
}

impl<T:RawNode> NodeRef for Option<T> {
    type TargetNode = NodeItem;

    fn to_owned(&self) -> Option<Self::TargetNode> {
        self.as_ref()
            .map(|n|n.inner())
    }
}

