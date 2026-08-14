use std::collections::LinkedList;
use super::*;

#[derive(Clone)]
pub struct ElementItem(NodeItem);

impl fmt::Display for ElementItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(debug_assertions)]
impl fmt::Debug for ElementItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl PartialEq<NodeItem> for ElementItem {
    fn eq(&self, other: &NodeItem) -> bool {
        other.eq(&self.0)
    }
}

impl From<NodeItem> for ElementItem {
    fn from(value: NodeItem) -> Self {
        Self(value)
    }
}

impl Deref for ElementItem {
    type Target = NodeItem;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ElementItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ElementItem {
    #[inline]
    pub(crate) fn inner_att(&self) -> &AttributeMap {
        self.0.inner()
            .attributes()
            .unwrap()
    }

    #[inline]
    pub(crate) fn inner_att_mut(&mut self) -> &mut AttributeMap {
        self.0.inner_mut()
            .attributes_mut()
            .unwrap()
    }
}

impl Element for ElementItem {
    #[inline]
    fn attributes(&self) -> AttributeIter<'_> {
        self.inner_att()
            .iter()
    }

    #[inline]
    fn attribute_names(&self) -> Vec<&String> {
        self.attributes()
            .names()
            .collect()
    }

    #[inline]
    fn has_attribute(&self, name:&str) -> bool {
        self.get_attribute(name)
            .is_some()
    }


    #[inline]
    fn set_attribute<N:ToString, V:Into<Attribute>>(&mut self, name:N, value:V) {
        self.inner_att_mut()
            .set_attribute(name, value);
    }

    #[inline]
    fn get_attribute(&self, name:&str) -> Option<&Attribute> {
        self.inner_att()
            .get_attribute(name)
    }

    #[inline]
    fn toggle_attribute<N:ToString>(&mut self, name:N, force:Option<bool>) {
        self.inner_att_mut()
            .toggle_attribute(name, force);
    }

    #[inline]
    fn get_class_name(&self) -> String {
        self.inner_att()
            .get_attribute("class")
            .map(|a|a.to_string())
            .unwrap_or(String::new())
    }

    #[inline]
    fn set_class_name<T:ToString>(&mut self, value:T) {
        self.inner_att_mut()
            .set_attribute("class", value.to_string());
    }

    #[inline]
    fn class_list(&self) -> Vec<String> {
        self.get_class_name()
            .split_whitespace()
            .map(|s|s.to_string())
            .collect()
    }

    #[inline]
    fn get_id(&self) -> String {
        self.get_attribute("id")
            .map(|a|a.to_string())
            .unwrap_or(String::new())
    }

    #[inline]
    fn set_id<T:ToString>(&mut self, value:T) {
        self.set_attribute("id", value.to_string());
    }

    #[inline]
    fn tag_name(&self) -> &str {
        Node::tag_name(&self.0)
    }

    #[inline]
    fn children(&self) -> NodeList<ElementItem> {
        self.0.child_nodes()
            .iter()
            .filter_map(|n| (n.node_type() == NodeType::Element).then(||{
                ElementItem(n.clone())
            }))
            .into()
    }

    fn first_child(&self) -> Option<ElementItem> {
        let mut it = self.0.child_nodes()
            .iter();

        while let Some(node) = it.next() {
            if node.node_type() == NodeType::Element {
                return Some(ElementItem(node.clone()))
            }
        }

        None
    }

    fn last_child(&self) -> Option<ElementItem> {
        let mut it = self.0.child_nodes()
            .iter();

        while let Some(node) = it.next_back() {
            if node.node_type() == NodeType::Element {
                return Some(ElementItem(node.clone()))
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
    fn parrent(&self) -> Option<ElementItem> {
        self.parrent_node()
            .map(|n|if n.node_type() == NodeType::Element {
                Some(ElementItem(n))
            } else {
                None
            }).flatten()
    }

    fn after<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: DerefMut<Target = NodeItem>
    {
        if let Some(parrent) = self.parrent() {

        } else {
            Err(NodeError::ElementNotConnected)
        }
    }

    fn before<N>(&mut self, node:&mut N)
        where N: DerefMut<Target = NodeItem>;
    fn append<E: Element>(&mut self, node:&mut E);
    fn prepend<E:Element>(&mut self, node:&mut E);
    fn remove(&mut self);
    fn replace_with<N>(&mut self, node:&mut N)
        where N: DerefMut<Target = NodeItem>;

    fn insert_adjacent_element<E:Element>(&mut self, adjacent_where:AdjacentWhere, element:E);
    fn insert_adjacent_text<S:ToString>(&mut self, adjacent_where:AdjacentWhere, element:S);

    fn bounding_box(&self) -> BoundingBox;
    fn elements_by_class_name<T:ToString>(&self, class_name:T);
    fn elements_by_tag_name<T:ToString>(&self, tag_name:T);
    fn elements_by_id<T:ToString>(&self, id:T);

    fn matches(&self, query_string:&Query) -> bool;
    fn query_selector(&self, query_string:&Query) -> Option<ElementItem>;
    fn query_selector_all(&self, query_string:&Query) -> QueryIterator<'_>;

    fn scroll_into_view(&mut self);
    fn scroll_by(&mut self, delta:f64);
}