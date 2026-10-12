use std::collections::LinkedList;
use super::*;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct DocumentFragment(NodeItem);

impl Default for DocumentFragment {
    fn default() -> Self {
        Self(NodeItem::new_root())
    }
}

impl RawNode for DocumentFragment {
    fn as_item(&self) -> NodeItem {
        self.0.clone()
    }
}

impl Node for DocumentFragment {
    #[inline]
    fn node_type(&self) -> NodeType {
        NodeType::Fragment
    }

    #[inline]
    fn tag_name(&self) -> &str {
        ""
    }

    #[inline]
    fn child_nodes(&self) -> &NodeList<impl Node> {
        self.0.inner()
            .children()
            .unwrap_or(&EMPTY)
    }

    #[inline]
    fn get_content(&self) -> String {
        self.0.inner().content()
    }

    #[inline]
    fn set_content<T:ToString>(&mut self, content:T) {
        self.0.inner_mut().set_content(&content.to_string());
    }

    #[inline]
    fn parrent_node(&self) -> impl NodeRef {
        self.0.inner().parrent()
    }

    fn contains(&self, node:&impl Node) -> bool {
        if node.is_node(&self.0) {
            return true;
        }

        self.0.inner().contains(node)
    }

    fn append_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: Node
    {
        let mut node = node.as_item();

        self.0.inner_mut()
            .append(&node)?;

        node.inner_mut()
            .set_parrent(&self.0);

        Ok(())
    }

    fn prepend_node<N>(&mut self, node:&mut N) -> Result<(), NodeError> 
        where N: Node
    {
        let mut node = node.as_item();

        self.0.inner_mut()
            .prepend(&node)?;

        node.inner_mut()
            .set_parrent(&self.0);

        Ok(())
    }

    fn insert_before<N, R>(&mut self, new_node:&mut N, ref_node:&R) -> Result<(), NodeError>
        where N: Node,
              R: Node
    {
        let mut new_node = new_node.as_item();
        let ref_node = ref_node.as_item();

        if self.0.inner_mut().insert_before(&new_node, &ref_node)? {
            new_node.inner_mut()
                .set_parrent(&self.0);

            Ok(())
        } else {
            Err(NodeError::NotDesendent(self.0.clone(), ref_node))
        }
    }

    fn insert_after<N, R>(&mut self, new_node:&mut N, ref_node:&R) -> Result<(), NodeError>
        where N: Node,
              R: Node
    {
        let mut new_node = new_node.as_item();
        let ref_node = ref_node.as_item();

        if self.0.inner_mut().insert_after(&new_node, &ref_node)? {
            new_node.inner_mut()
                .set_parrent(&self.0);

            Ok(())
        } else {
            Err(NodeError::NotDesendent(self.0.clone(), ref_node))
        }
    }

    fn remove_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: Node
    {
        let mut node = node.as_item();

        if self.0.inner_mut().remove(node.inner()) {
            node.inner_mut().remove_parrent();
            Ok(())
        } else {
            Err(NodeError::NotDesendent(self.0.clone(), node.clone()))
        }
    }

    fn is_connected(&self) -> bool {
        false
    }
}

impl Element for DocumentFragment {
    #[inline]
    fn attributes(&self) -> AttributeIter<'_> {
        self.0.inner_att()
            .iter()
    }

    #[inline]
    fn attribute_names(&self) -> Vec<&String> {
        self.0.attributes()
            .names()
            .collect()
    }

    #[inline]
    fn set_attribute<N:ToString, V:Into<Attribute>>(&mut self, name:N, value:V) {
        self.0.inner_att_mut()
            .set_attribute(name, value);
    }

    #[inline]
    fn get_attribute(&self, name:&str) -> Option<&Attribute> {
        self.0.inner_att()
            .get_attribute(name)
    }

    #[inline]
    fn toggle_attribute<N:ToString>(&mut self, name:N) {
        self.0.inner_att_mut()
            .toggle_attribute(name, None);
    }

    #[inline]
    fn force_toggle_attribute<N:ToString>(&mut self, name:N, force:bool) {
        self.0.inner_att_mut()
            .toggle_attribute(name, Some(force));
    }

    #[inline]
    fn get_class_name(&self) -> String {
        self.0.inner_att()
            .get_attribute("class")
            .map(|a|a.to_string())
            .unwrap_or(String::new())
    }

    #[inline]
    fn set_class_name<T:ToString>(&mut self, value:T) {
        self.0.inner_att_mut()
            .set_attribute("class", value.to_string());
    }

    #[inline]
    fn class_list(&self) -> Vec<String> {
        self.0.get_class_name()
            .split_whitespace()
            .map(|s|s.to_string())
            .collect()
    }

    #[inline]
    fn get_id(&self) -> String {
        self.0.get_attribute("id")
            .map(|a|a.to_string())
            .unwrap_or(String::new())
    }

    #[inline]
    fn set_id<T:ToString>(&mut self, value:T) {
        self.0.set_attribute("id", value.to_string());
    }

    #[inline]
    fn children(&self) -> NodeList<impl Element> {
        self.0.child_nodes()
            .iter()
            .filter_map(|n| (n.node_type() == NodeType::Element).then(||{
                n.as_item()
            }))
            .into()
    }

    fn first_child(&self) -> Option<impl Element> {
        let mut it = self.0.child_nodes()
            .iter();

        while let Some(node) = it.next() {
            if node.node_type() == NodeType::Element {
                return Some(node.as_item())
            }
        }

        None
    }

    fn last_child(&self) -> Option<impl Element> {
        let mut it = self.0.child_nodes()
            .iter();

        while let Some(node) = it.next_back() {
            if node.node_type() == NodeType::Element {
                return Some(node.as_item())
            }
        }

        None
    }

    fn client_height(&self) -> u32 {
        0
    }

    fn client_left(&self) -> u32 {
        0
    }

    fn client_top(&self) -> u32 {
        0
    }

    fn client_width(&self) -> u32 {
        0
    }

    fn current_zoom(&self) -> f64 {
        0.0
    }

    fn scroll_height(&self) -> u32 {
        0
    }

    fn scroll_left(&self) -> u32 {
        0
    }

    fn scroll_top(&self) -> u32 {
        0
    }

    fn scroll_bottom(&self) -> u32 {
        0
    }

    #[inline]
    fn parrent(&self) -> Option<impl Element> {
        self.0.parrent_node()
            .to_owned()
            .map(|n|{
                let n = n.as_item();
                if n.node_type() == NodeType::Element {
                    Some(n)
                } else {
                    None
                }
        }).flatten()
    }

    fn after(&mut self, node:&impl RawNode) -> Result<(), NodeError> {
        if let Some(mut parrent) = self.parrent() {
            parrent.append_node(&mut node.as_item())
        } else {
            Err(NodeError::NodeIsNotConnected(self.0.clone()))
        }
    }

    fn before(&mut self, node:&impl RawNode) -> Result<(), NodeError> {
        if let Some(mut parrent) = self.parrent() {
            parrent.prepend_node(&mut node.as_item())
        } else {
            Err(NodeError::NodeIsNotConnected(self.0.clone()))
        }
    }

    fn append(&mut self, child:&impl RawNode) -> Result<(), NodeError> {
        self.append_node(&mut child.as_item())
    }

    fn prepend(&mut self, child:&impl RawNode) -> Result<(), NodeError> {
        self.prepend_node(&mut child.as_item())
    }

    fn remove(&mut self) -> Result<(), NodeError> {
        if let Some(parrent) = self.parrent() {
            let mut parrent = parrent.as_item();
            if parrent.inner_mut()
                .remove(self.0.inner())
            {
                Ok(())
            } else {
                Err(NodeError::NotDesendent(parrent, self.0.clone()))
            }
                
        } else {
            Err(NodeError::NodeIsNotConnected(self.0.clone()))
        }
    }

    fn replace_with(&mut self, node:&impl RawNode) -> Result<(), NodeError> {
        if let Some(parrent) = self.parrent() {
            let mut parrent = parrent.as_item();

            if parrent.inner_mut().insert_after(&node.as_item(), &self.0)?
                && parrent.inner_mut().remove(&self.0.inner())
            {
                return Ok(());
            }
        }

        Err(NodeError::NodeIsNotConnected(self.0.clone()))
    }

    fn insert_adjacent_element(&mut self, adjacent_where:AdjacentWhere, element:&impl Element) -> Result<(), NodeError> {
        let mut node = element.as_item();
        
        match adjacent_where {
            AdjacentWhere::AfterBegin => 
                self.prepend_node(&mut node),
            AdjacentWhere::BeforeBegin => if let Some(mut parrent) = self.parrent() {
                parrent.insert_before(&mut node, self)
            } else {
                Err(NodeError::NodeIsNotConnected(self.0.clone()))
            },
            AdjacentWhere::BeforeEnd =>
                self.append_node(&mut node),
            AdjacentWhere::AfterEnd => if let Some(mut parrent) = self.parrent() {
                parrent.insert_after(&mut node, self)
            } else {
                Err(NodeError::NodeIsNotConnected(self.0.clone()))
            },
        }
    }

    fn insert_adjacent_text(&mut self, adjacent_where:AdjacentWhere, text:&impl ToString) -> Result<(), NodeError> {
        let mut node = text.to_string().as_item();

        match adjacent_where {
            AdjacentWhere::AfterBegin => 
                self.prepend_node(&mut node),
            AdjacentWhere::BeforeBegin => if let Some(mut parrent) = self.parrent() {
                parrent.insert_before(&mut node, self)
            } else {
                Err(NodeError::NodeIsNotConnected(self.0.clone()))
            },
            AdjacentWhere::BeforeEnd =>
                self.append_node(&mut node),
            AdjacentWhere::AfterEnd => if let Some(mut parrent) = self.parrent() {
                parrent.insert_after(&mut node, self)
            } else {
                Err(NodeError::NodeIsNotConnected(self.0.clone()))
            }
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        BoundingBox{
            top: 0,
            bottom: 0,
            left: 0,
            right: 0
        }
    }

    #[inline]
    fn elements_by_class_name(&self, class_name:&str) -> NodeList<impl Element> {
        self.0.children_map(|elm|elm.get_class_name().eq(class_name))
    }

    #[inline]
    fn elements_by_tag_name(&self, tag_name:&str) -> NodeList<impl Element> {
        self.0.children_map(|elm|elm.tag_name().eq(tag_name))
    }

    #[inline]
    fn elements_by_id(&self, id:&str) -> NodeList<impl Element> {
        self.0.children_map(|elm|elm.get_id().eq(id))
    }

    #[inline]
    fn matches<Q: Query>(&self, query:&Q) -> Result<bool, QueryError> {
        query.match_element(self)
    }

    fn query_selector<Q: Query>(&self, query:&Q) -> Result<Option<impl Element>, QueryError> {
        let query = query.parse()?;
        let list = self.0.children_inner();

        for child in list.iter() {
            if child.matches(&query)? {
                return Ok(Some(child.clone()))
            }
        }

        for child in list.iter() {
            if let Some(result) = child.query_selector(&query)? {
                return Ok(Some(result.as_item()))
            }
        }

        Ok(None)
    }

    fn query_selector_all<Q: Query>(&self, query:&Q) -> Result<NodeList<impl Element>, QueryError> {
        let mut list:LinkedList<NodeItem> = LinkedList::new();
        let query = query.parse()?;
        let children = self.0.children_inner();

        for child in children.iter() {
            if child.matches(&query)? {
                list.push_back(child.clone())
            }

            if let Some(inner_children) = child.query_selector_all(&query)?.0 {
                for inner_child in inner_children {
                    list.push_back(inner_child.as_item());
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