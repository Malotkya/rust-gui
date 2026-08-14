use std::fmt;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    ops::{Deref, DerefMut}
};
use super::*;

#[derive(Clone)]
pub struct NodeItem(Rc<RefCell<NodeInner>>);

impl fmt::Display for NodeItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.inner(), f)
    }
}

#[cfg(debug_assertions)]
impl fmt::Debug for NodeItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.inner(), f)
    }
}

impl PartialEq for NodeItem {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            self.0.as_ptr(),
            other.0.as_ptr()
        )
    }
}

impl Eq for NodeItem {}

impl PartialEq<NodeInner> for NodeItem {
    fn eq(&self, other: &NodeInner) -> bool {
        std::ptr::eq(
            self.inner(),
            other
        )
    }
}

impl PartialEq<NodeItem> for NodeInner {
    fn eq(&self, other: &NodeItem) -> bool {
        std::ptr::eq(
            self,
            other.inner()
        )
    }
}

impl NodeItem {
    pub(crate) fn inner(&self) -> &NodeInner {
        unsafe{ &*self.0.as_ptr() }
    }

    pub(crate) fn inner_mut(&mut self) -> &mut NodeInner {
        unsafe{ &mut *self.0.as_ptr() }
    }

    pub fn new_text<T:ToString>(text:T) -> Self {
        Self(Rc::new(
            RefCell::new(
                NodeInner::new_text(text.to_string())
            )
        ))
    }

    pub fn new_element<T:ToString>(tag_name:T) -> Self {
        Self(Rc::new(
            RefCell::new(
                NodeInner::new_element(tag_name.to_string())
            )
        ))
    }

    pub fn as_ref(&self) -> NodeRef {
        NodeRef(
            Rc::downgrade(&self.0)
        )
    }
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct NodeRef(Weak<RefCell<NodeInner>>);

impl NodeRef {
    #[inline]
    pub fn node(&self) -> Option<NodeItem> {
        self.0.upgrade()
            .map(|i|NodeItem(i))
    }
}

impl Node for NodeItem {
    #[inline]
    fn node_type(&self) -> NodeType {
        self.inner().node_type()
    }

    #[inline]
    fn tag_name(&self) -> &str {
        self.inner().tag_name()
    }

    #[inline]
    fn child_nodes(&self) -> &NodeList<NodeItem> {
        self.inner()
            .children()
            .unwrap_or(&EMPTY)
    }

    #[inline]
    fn get_content(&self) -> String {
        self.inner().content()
    }

    #[inline]
    fn set_content<T:ToString>(&mut self, content:T) {
        self.inner_mut().set_content(content.to_string());
    }

    #[inline]
    fn parrent_node(&self) -> Option<NodeItem> {
        self.inner().parrent()
    }

    fn contains<T:PartialEq<NodeItem>>(&self, node:&T) -> bool {
        if node.eq(self) {
            return true;
        }

        self.inner().contains(node)
    }

    fn append_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: DerefMut<Target = NodeItem>
    {
        self.inner_mut()
            .append(node)?;

        node.deref_mut().inner_mut()
            .set_parrent(self);

        Ok(())
    }

    fn prepend_node<N>(&mut self, node:&mut N) -> Result<(), NodeError> 
        where N: DerefMut<Target = NodeItem>
    {
        self.inner_mut()
            .prepend(node)?;

        node.deref_mut().inner_mut()
            .set_parrent(self);

        Ok(())
    }

    fn insert_before<N, R>(&mut self, new_node:&mut N, ref_node:&R) -> Result<(), NodeError>
        where N: DerefMut<Target = NodeItem>,
              R: Deref<Target = NodeItem>
    {
        if self.inner_mut().insert(new_node, ref_node)? {
            new_node.deref_mut().inner_mut()
                .set_parrent(self);

            Ok(())
        } else {
            Err(NodeError::NotDesendent(self.clone(), (*ref_node).clone()))
        }
    }

    fn remove_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: DerefMut<Target = NodeItem>
    {
        if self.inner_mut().remove(node.inner()) {
            node.inner_mut().remove_parrent();
            Ok(())
        } else {
            Err(NodeError::NotDesendent(self.clone(), node.clone()))
        }
    }

    fn node(&self) -> NodeItem {
        self.clone()
    }
}