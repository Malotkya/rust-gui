use super::*;
use std::collections::{
    LinkedList,
    linked_list::{Iter, IterMut}
};

pub(super) const EMPTY:NodeList<NodeItem> = NodeList(None);

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct NodeList<N: Node>(Option<LinkedList<N>>);

impl<N:Node, I:Iterator<Item = N>> From<I> for NodeList<N> {
    fn from(value: I) -> Self {
        Self(Some(
            value.collect()
        ))
    }
}

impl<N: Node> NodeList<N> {
    pub fn iter(&self) -> NodeIter<'_, N> {
        if let Some(inner) = &self.0 {
            inner.iter().into()
        } else {
            NodeIter::default()
        }
    }

    pub fn at(&self, index:usize) -> Option<&N> {
        if let Some(inner) = &self.0 {
            inner.iter()
                .nth(index)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        if let Some(inner) = &self.0 {
            inner.len()
        } else {
            0
        }
    }
}

impl<N:Node> NodeList<N> {
    pub(super) const fn new() -> Self {
        Self(Some(LinkedList::new()))
    }

    pub(super) fn insert_end<I: Into<N>>(&mut self, node:I) {
        self.0.as_mut()
            .unwrap()
            .push_back(node.into());
    }

    pub(super) fn insert_start<I: Into<N>>(&mut self, node:I) {
        self.0.as_mut()
            .unwrap()
            .push_front(node.into());
    }

    pub(super) fn insert_before<I: Into<N>, R:PartialEq<N>>(&mut self, new_node:I, ref_node:&R) -> bool {
        let inner = self.0.as_mut().unwrap();
        if let Some(index) = Self::find_node(&inner, ref_node) {
            let mut split = inner.split_off(index);
            split.push_front(new_node.into());
            inner.append(&mut split);

            true
        } else {
            false
        }
    }

    fn find_node<T:PartialEq<N>>(list:&LinkedList<N>, node:&T) -> Option<usize> {
        for (index, child) in list.iter().enumerate() {
            if node.eq(child) {
                return Some(index);
            }
        }

        None
    }

    pub(super) fn remove_node<T:PartialEq<N>>(&mut self, node:&T) -> bool {
        let inner = self.0.as_mut().unwrap();

        if let Some(index) = Self::find_node(&inner, node) {
            let mut split = inner.split_off(index);
            split.pop_front();
            inner.append(&mut split);

            true
        } else {
            false
        }
    }

    pub(super) fn clear_after(&mut self, index:usize, remove:impl Fn(&N) -> bool) {
        let inner = self.0.as_mut().unwrap();

        let mut split = inner.split_off(index);
        split = split
            .into_iter()
            .filter(|n|!(remove(n)))
            .collect();
        inner.append(&mut split);
    }

    pub(super) fn iter_mut(&mut self) -> NodeIterMut<'_, N> {
        self.0.as_mut()
            .unwrap()
            .iter_mut()
            .into()
    }

    pub(super) fn content(&self) -> Vec<String> {
        self.iter()
            .map(|n|n.get_content().trim().to_string())
            .collect::<Vec<_>>()
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct NodeIter<'a, N:Node>(Option<Iter<'a, N>>);

impl<'a, N:Node> Iterator for NodeIter<'a, N> {
    type Item = &'a N;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut()
            .map(|it|it.next())
            .flatten()
    }
}

impl<'a, N:Node> DoubleEndedIterator for NodeIter<'a, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.as_mut()
            .map(|it|it.next_back())
            .flatten()
    }
}

impl<'a, N:Node> From<Iter<'a, N>> for NodeIter<'a, N> {
    fn from(value: Iter<'a, N>) -> Self {
        Self(Some(value))
    }
}

impl<'a, N:Node> Default for NodeIter<'a, N> {
    fn default() -> Self {
        Self(None)
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct NodeIterMut<'a, N:Node>(Option<IterMut<'a, N>>);

impl<'a, N:Node> Iterator for NodeIterMut<'a, N> {
    type Item = &'a mut N;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut()
            .map(|it|it.next())
            .flatten()
    }
}

impl<'a, N:Node> DoubleEndedIterator for NodeIterMut<'a, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.as_mut()
            .map(|it|it.next_back())
            .flatten()
    }
}

impl<'a, N:Node> From<IterMut<'a, N>> for NodeIterMut<'a, N> {
    fn from(value: IterMut<'a, N>) -> Self {
        Self(Some(value))
    }
}

impl<'a, N:Node> Default for NodeIterMut<'a, N> {
    fn default() -> Self {
        Self(None)
    }
}