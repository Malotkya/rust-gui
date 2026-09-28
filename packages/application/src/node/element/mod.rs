use std::fmt;
use super::*;

mod attribute;
pub use attribute::*;
mod attribute_map;
pub use attribute_map::*;
mod query;
pub use query::*;

#[derive(Clone)]
pub struct Element(NodeItem);

#[derive(Debug, Clone)]
pub enum AdjacentWhere {
    BeforeBegin,
    AfterBegin,
    BeforeEnd,
    AfterEnd
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    top: u32,
    bottom: u32
}

use crate::node::Node;
use std::collections::LinkedList;

impl Element {
    fn children_map(&self, func: impl Fn(&Element)->bool) -> NodeList<Element> {
        self.0.child_nodes()
            .iter()
            .filter_map(|n| if n.node_type() == NodeType::Element {
                let elm = Element(n.inner());
                if func(&elm) {
                    Some(elm)
                } else {
                    None
                }
            } else {
                None
            })
            .into()
    }

    fn children_inner(&self) -> NodeList<Element> {
        self.0.child_nodes()
            .iter()
            .filter_map(|n| (n.node_type() == NodeType::Element).then(||{
                Element(n.inner())
            }))
            .into()
    }

    #[inline]
    fn inner_att(&self) -> &AttributeMap {
        self.0.inner()
            .attributes()
            .unwrap()
    }

    #[inline]
    fn inner_att_mut(&mut self) -> &mut AttributeMap {
        self.0.inner_mut()
            .attributes_mut()
            .unwrap()
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(debug_assertions)]
impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl PartialEq<NodeItem> for Element {
    fn eq(&self, other: &NodeItem) -> bool {
        other.eq(&self.0)
    }
}

impl From<NodeItem> for Element {
    fn from(value: NodeItem) -> Self {
        Self(value)
    }
}

impl RawNode for Element {
    fn inner(&self) -> NodeItem {
        self.0.clone()
    }
}

impl Node for Element {
    #[inline]
    fn node_type(&self) -> NodeType {
        NodeType::Element
    }

    #[inline]
    fn tag_name(&self) -> &str {
        self.0.tag_name()
    }

    #[inline]
    fn child_nodes(&self) -> &NodeList<impl Node> {
        self.0.child_nodes()
    }

    #[inline]
    fn get_content(&self) -> String {
        self.0.get_content()
    }

    #[inline]
    fn set_content<T:ToString>(&mut self, content:T) {
        self.0.set_content(content);
    }

    #[inline]
    fn parrent_node(&self) -> impl NodeRef {
        self.0.parrent_node()
    }

    #[inline]
    fn contains<T:PartialEq<NodeItem>>(&self, node:&T) -> bool {
        self.0.contains(node)
    }

    #[inline]
    fn append_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: Node
    {
        self.0.append_node(node)
    }

    #[inline]
    fn prepend_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: Node
    {
        self.0.prepend_node(node)
    }

    #[inline]
    fn insert_before<N, R>(&mut self, new_node:&mut N, ref_node:&R) -> Result<(), NodeError>
        where N: Node,
              R: Node
    {
        self.0.insert_before(new_node, ref_node)
    }

    #[inline]
    fn insert_after<N, R>(&mut self, new_node:&mut N, ref_node:&R) -> Result<(), NodeError>
        where N: Node,
              R: Node
    {
        self.0.insert_after(new_node, ref_node)
    }

    #[inline]
    fn remove_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: Node
    {
        self.0.remove_node(node)
    }

    #[inline]
    fn is_connected(&self) -> bool {
        self.0.is_connected()
    }
}

impl Element {
    #[inline]
    pub fn attributes(&self) -> AttributeIter<'_> {
        self.inner_att()
            .iter()
    }

    #[inline]
    pub fn attribute_names(&self) -> Vec<&String> {
        self.attributes()
            .names()
            .collect()
    }

    #[inline]
    pub fn has_attribute(&self, name:&str) -> bool {
        self.get_attribute(name)
            .is_some()
    }


    #[inline]
    pub fn set_attribute<N:ToString, V:Into<Attribute>>(&mut self, name:N, value:V) {
        self.inner_att_mut()
            .set_attribute(name, value);
    }

    #[inline]
    pub fn get_attribute(&self, name:&str) -> Option<&Attribute> {
        self.inner_att()
            .get_attribute(name)
    }

    #[inline]
    pub fn toggle_attribute<N:ToString>(&mut self, name:N) {
        self.inner_att_mut()
            .toggle_attribute(name, None);
    }

    #[inline]
    pub fn force_toggle_attribute<N:ToString>(&mut self, name:N, force:bool) {
        self.inner_att_mut()
            .toggle_attribute(name, Some(force));
    }

    #[inline]
    pub fn get_class_name(&self) -> String {
        self.inner_att()
            .get_attribute("class")
            .map(|a|a.to_string())
            .unwrap_or(String::new())
    }

    #[inline]
    pub fn set_class_name<T:ToString>(&mut self, value:T) {
        self.inner_att_mut()
            .set_attribute("class", value.to_string());
    }

    #[inline]
    pub fn class_list(&self) -> Vec<String> {
        self.get_class_name()
            .split_whitespace()
            .map(|s|s.to_string())
            .collect()
    }

    #[inline]
    pub fn get_id(&self) -> String {
        self.get_attribute("id")
            .map(|a|a.to_string())
            .unwrap_or(String::new())
    }

    #[inline]
    pub fn set_id<T:ToString>(&mut self, value:T) {
        self.set_attribute("id", value.to_string());
    }

    #[inline]
    pub fn children(&self) -> NodeList<Element> {
        self.0.child_nodes()
            .iter()
            .filter_map(|n| (n.node_type() == NodeType::Element).then(||{
                Element(n.inner())
            }))
            .into()
    }

    pub fn first_child(&self) -> Option<Element> {
        let mut it = self.0.child_nodes()
            .iter();

        while let Some(node) = it.next() {
            if node.node_type() == NodeType::Element {
                return Some(Element(node.inner()))
            }
        }

        None
    }

    fn last_child(&self) -> Option<Element> {
        let mut it = self.0.child_nodes()
            .iter();

        while let Some(node) = it.next_back() {
            if node.node_type() == NodeType::Element {
                return Some(Element(node.inner()))
            }
        }

        None
    }

    fn client_height(&self) -> u32 {
        todo!("Dimensions & Styling")
    }

    fn client_left(&self) -> u32{
        todo!("Dimensions & Styling")
    }

    fn client_top(&self) -> u32{
        todo!("Dimensions & Styling")
    }

    fn client_width(&self) -> u32 {
        todo!("Dimensions & Styling")
    }

    fn current_zoom(&self) -> f64 {
        todo!("Dimensions & Styling")
    }

    fn scroll_height(&self) -> u32 {
        todo!("Dimensions & Styling")
    }

    fn scroll_left(&self) -> u32 {
        todo!("Dimensions & Styling")
    }

    fn scroll_top(&self) -> u32 {
        todo!("Dimensions & Styling")
    }

    fn scroll_bottom(&self) -> u32 {
        todo!("Dimensions & Styling")
    }

    #[inline]
    fn parrent(&self) -> Option<Element> {
        self.parrent_node()
            .to_owned()
            .map(|n|{
                let n = n.inner();
                if n.node_type() == NodeType::Element {
                    Some(Element(n))
                } else {
                    None
                }
        }).flatten()
    }

    fn after(&mut self, node:&impl RawNode) -> Result<(), NodeError> {
        if let Some(mut parrent) = self.parrent() {
            parrent.append_node(&mut node.inner())
        } else {
            Err(NodeError::NodeIsNotConnected(self.inner()))
        }
    }

    fn before(&mut self, node:&impl RawNode) -> Result<(), NodeError> {
        if let Some(mut parrent) = self.parrent() {
            parrent.prepend_node(&mut node.inner())
        } else {
            Err(NodeError::NodeIsNotConnected(self.inner()))
        }
    }

    fn append(&mut self, child:&impl RawNode) {
        self.append_node(&mut child.inner()).unwrap()
    }

    fn prepend(&mut self, child:&impl RawNode) {
        self.prepend_node(&mut child.inner()).unwrap()
    }

    fn remove(&mut self) -> Result<(), NodeError> {
        if let Some(parrent) = self.parrent() {
            let mut parrent = RawNode::inner(&parrent);
            let mut this = self.inner();

            if parrent.inner_mut()
                .remove(&mut this)
            {
                Ok(())
            } else {
                Err(NodeError::NotDesendent(parrent, this))
            }
                
        } else {
            Err(NodeError::NodeIsNotConnected(self.inner()))
        }
    }

    fn replace_with(&mut self, node:&impl RawNode) -> Result<(), NodeError> {
        if let Some(parrent) = self.parrent() {
            let mut parrent = RawNode::inner(&parrent);

            if parrent.inner_mut().insert_after(&node.inner(), &self.inner())?
                && parrent.inner_mut().remove(&mut self.inner())
            {
                return Ok(());
            }
        }

        Err(NodeError::NodeIsNotConnected(self.inner()))
    }

    fn insert_adjacent_element(&mut self, adjacent_where:AdjacentWhere, element:&Element) -> Result<(), NodeError>{
        let mut node = RawNode::inner(element);
        
        match adjacent_where {
            AdjacentWhere::AfterBegin => 
                self.prepend_node(&mut node),
            AdjacentWhere::BeforeBegin => if let Some(mut parrent) = self.parrent() {
                parrent.insert_before(&mut node, &self.inner())
            } else {
                Err(NodeError::NodeIsNotConnected(self.inner()))
            },
            AdjacentWhere::BeforeEnd =>
                self.append_node(&mut node),
            AdjacentWhere::AfterEnd => if let Some(mut parrent) = self.parrent() {
                parrent.insert_after(&mut node, &self.inner())
            } else {
                Err(NodeError::NodeIsNotConnected(self.inner()))
            },
        }
    }

    fn insert_adjacent_text(&mut self, adjacent_where:AdjacentWhere, text:&impl ToString) -> Result<(), NodeError> {
        let mut node = text.to_string().inner();

        match adjacent_where {
            AdjacentWhere::AfterBegin => 
                self.prepend_node(&mut node),
            AdjacentWhere::BeforeBegin => if let Some(mut parrent) = self.parrent() {
                parrent.insert_before(&mut node, &self.inner())
            } else {
                Err(NodeError::NodeIsNotConnected(self.inner()))
            },
            AdjacentWhere::BeforeEnd =>
                self.append_node(&mut node),
            AdjacentWhere::AfterEnd => if let Some(mut parrent) = self.parrent() {
                parrent.insert_after(&mut node, &self.inner())
            } else {
                Err(NodeError::NodeIsNotConnected(self.inner()))
            }
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        todo!("Dimensions & Styling")
    }

    fn elements_by_class_name(&self, class_name:&str) -> NodeList<Element> {
        self.children_map(|elm|elm.get_class_name().eq(class_name))
    }

    fn elements_by_tag_name(&self, tag_name:&str) -> NodeList<Element> {
        self.children_map(|elm|elm.tag_name().eq(tag_name))
    }

    fn elements_by_id(&self, id:&str) -> NodeList<Element> {
        self.children_map(|elm|elm.get_id().eq(id))
    }

    fn matches<Q: Query>(&self, query:&Q) -> Result<bool, QueryError> {
        query.match_element(self)
    }

    fn query_selector<Q: Query>(&self, query:&Q) -> Result<Option<Element>, QueryError> {
        let query = query.parse()?;
        let list = self.children_inner();

        for child in list.iter() {
            if child.matches(&query)? {
                return Ok(Some(child.clone()))
            }
        }

        for child in list.iter() {
            if let Some(result) = child.query_selector(&query)? {
                return Ok(Some(result))
            }
        }

        Ok(None)
    }

    fn query_selector_all<Q: Query>(&self, query:&Q) -> Result<NodeList<Element>, QueryError> {
        let mut list:LinkedList<Element> = LinkedList::new();
        let query = query.parse()?;
        let children = self.children_inner();

        for child in children.iter() {
            if child.matches(&query)? {
                list.push_back(child.clone())
            }

            if let Some(inner_children) = child.query_selector_all(&query)?.0 {
                for inner_child in inner_children {
                    list.push_back(inner_child);
                }
            }
        }

        Ok(NodeList(Some(list)))
    }

    fn scroll_into_view(&mut self) {
        todo!("Interactivity")
    }

    fn scroll_by(&mut self, _delta:f64) {
        todo!("Interactivity")
    }
}

