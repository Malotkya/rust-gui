use std::collections::linked_list::{Iter, IterMut};
use super::NodeRef;

pub struct NodeIterator<'a>(Option<Iter<'a, NodeRef>>);

impl<'a> Default for NodeIterator<'a> {
    fn default() -> Self {
        Self(None)
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a NodeRef;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut()
            .map(|it|it.next())
            .flatten()
    }
}

impl<'a> From<Iter<'a, NodeRef>> for NodeIterator<'a> {
    fn from(value: Iter<'a, NodeRef>) -> Self {
        Self(Some(value))
    }
}

pub struct NodeIteratorMut<'a>(Option<IterMut<'a, NodeRef>>);

impl<'a> Iterator for NodeIteratorMut<'a> {
    type Item = &'a mut NodeRef;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut()
            .map(|it|it.next())
            .flatten()
    }
}

impl<'a> From<IterMut<'a, NodeRef>> for NodeIteratorMut<'a> {
    fn from(value: IterMut<'a, NodeRef>) -> Self {
        Self(Some(value))
    }
}

impl<'a> Default for NodeIteratorMut<'a> {
    fn default() -> Self {
        Self(None)
    }
}