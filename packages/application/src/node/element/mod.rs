use std::{
    collections::{
        HashMap, LinkedList
    },
    fmt
};
use super::*;

mod attribute;
pub use attribute::*;

#[derive(Clone)]
pub(crate) struct ElementData {
    tage_name:String,
    parrent: Option<NodeRef>,
    attributes: HashMap<String, Attribute>,
    children: LinkedList<NodeRef>,
}

impl ElementData {
    fn children_content(&self) -> Vec<String> {
        self.children()
            .map(|n|n.content())
            .collect::<Vec<_>>()
    }

    fn index_of(&self, node:&impl Node) -> Option<usize> {
        self.children()
            .enumerate()
            .find_map(|(i, n)| n.as_ref().eq(&node.as_ref()).then_some(i))
    }
}

impl fmt::Display for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct(&self.tage_name);
        for (key, value) in &self.attributes {
            dbg.field(key, &value.to_string());
        }

        dbg.field("children", &self.children);
        dbg.finish()
    }
}

#[cfg(debug_assertions)]
impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Node for ElementData {
    fn node_type(&self) -> u32 {
        0
    }

    fn tag_name(&self) -> &str {
        &self.tage_name
    }

    fn children(&self) -> NodeIterator<'_> {
        self.children.iter().into()
    }

    fn content(&self) -> String {
        self.children_content()
            .iter()
            .map(|c|c.trim())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn stringify(&self) -> String {
        format!("{}", self)
    }

    fn parrent(&self) -> Option<&NodeRef> {
        self.parrent.as_ref()
    }

    fn contains(&self, other:&impl Node) -> bool {
        if self.eq(&other.as_ref()) {
            return true;
        }

        for child in &self.children {
            if child.contains(other) {
                return true;
            }
        }

        return false;
    }

    fn append(&mut self, other:&impl Node) {
        let mut other = other.as_ref();
        other.set_parrent(self);
        self.children.push_back(other);
    }

    fn remove(&mut self, other:&impl Node) -> Result<(), NodeError> {
        let mut other = other.as_ref();
        if self.remove_child(&other) {
            other.remove_parrent();
            Ok(())
        } else {
            Err(
                NodeError::NotDesendent(self.as_ptr(), NodeCmp::as_ptr(&other))
            )
        }
    }

    fn children_mut(&mut self) -> NodeIteratorMut<'_> {
        self.children.iter_mut().into()
    }

    fn set_content<C:ToString>(&mut self, value:C) {
        let mut clear = -1;
        for (index, child) in self.children_mut().enumerate() {
            if child.node_type() == TextData::VALUE {
                child.set_content(value);
                clear = index as i32;
                break;
            }
        }

        if clear >= 0 {
            let mut split = self.children.split_off(clear as usize + 1);
            split = split.into_iter()
                .filter(|n|n.node_type() == TextData::VALUE)
                .collect();

            self.children.append(&mut split);
        } else {
            self.children.push_back(
                todo!("New Text Node!")
            )
        }
    }

    fn as_ref(&self) -> NodeRef {
        todo!("Get Reference!")
    }
}

impl NodeMut for ElementData {
    fn set_parrent(&mut self, value:&impl NodeMut) {
        let this = self.clone();
        if let Some(mut parrent) = this.parrent {
            parrent.remove_child(self);
        }
        
        self.parrent = Some(value.as_ref())
    }

    fn remove_parrent(&mut self) {
        let this = self.clone();
        if let Some(mut parrent) = this.parrent {
            parrent.remove_child(self);
        }

        self.parrent = None;
    }

    fn remove_child(&mut self, node:&impl NodeMut) -> bool {
        if let Some(index) = self.index_of(node) {
            let mut split = self.children.split_off(index);
            split.pop_front();
            self.children.append(&mut split);
            true
        } else {
            false
        }
    }
}

impl NodeCmp for ElementData {}