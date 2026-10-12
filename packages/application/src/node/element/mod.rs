use std::collections::LinkedList;
use super::*;

mod attribute;
pub use attribute::*;
mod attribute_map;
pub use attribute_map::*;
mod query;
pub use query::*;

#[derive(Debug, Clone)]
pub enum AdjacentWhere {
    BeforeBegin,
    AfterBegin,
    BeforeEnd,
    AfterEnd
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub top: u32,
    pub bottom: u32,
    pub left: u32,
    pub right: u32
}

pub trait Element: Node {
    fn attributes(&self) -> AttributeIter<'_>;
    fn attribute_names(&self) -> Vec<&String>;
    fn set_attribute<N:ToString, V:Into<Attribute>>(&mut self, name:N, value:V);
    fn get_attribute(&self, name:&str) -> Option<&Attribute>;
    fn toggle_attribute<N:ToString>(&mut self, name:N);
    fn force_toggle_attribute<N:ToString>(&mut self, name:N, force:bool);

    #[inline]
    fn has_attribute(&self, name:&str) -> bool {
        self.get_attribute(name)
            .is_some()
    }

    fn get_class_name(&self) -> String;
    fn set_class_name<T:ToString>(&mut self, value:T);
    fn class_list(&self) -> Vec<String>;
    fn get_id(&self) -> String;
    fn set_id<T:ToString>(&mut self, value:T);

    fn children(&self) -> NodeList<impl Element>;
    fn first_child(&self) -> Option<impl Element>;
    fn last_child(&self)  -> Option<impl Element>;
    fn parrent(&self) -> Option<impl Element>;
    fn after(&mut self, node:&impl RawNode) -> Result<(), NodeError>;
    fn before(&mut self, node:&impl RawNode) -> Result<(), NodeError>;
    fn append(&mut self, child:&impl RawNode) -> Result<(), NodeError>;
    fn prepend(&mut self, child:&impl RawNode) -> Result<(), NodeError>;
    fn remove(&mut self) -> Result<(), NodeError>;
    fn replace_with(&mut self, node:&impl RawNode) -> Result<(), NodeError>;
    fn insert_adjacent_element(&mut self, adjacent_where:AdjacentWhere, element:&impl Element) -> Result<(), NodeError>;
    fn insert_adjacent_text(&mut self, adjacent_where:AdjacentWhere, text:&impl ToString) -> Result<(), NodeError>;

    fn client_height(&self) -> u32;
    fn client_left(&self) -> u32;
    fn client_top(&self) -> u32;
    fn client_width(&self) -> u32;
    fn current_zoom(&self) -> f64;
    fn scroll_height(&self) -> u32;
    fn scroll_left(&self) -> u32;
    fn scroll_top(&self) -> u32;
    fn scroll_bottom(&self) -> u32;
    fn bounding_box(&self) -> BoundingBox;
    fn scroll_into_view(&mut self);
    fn scroll_by(&mut self, _delta:f64);

    fn elements_by_class_name(&self, class_name:&str) -> NodeList<impl Element>;
    fn elements_by_tag_name(&self, tag_name:&str) -> NodeList<impl Element>;
    fn elements_by_id(&self, id:&str) -> NodeList<impl Element>;
    fn matches<Q: Query>(&self, query:&Q) -> Result<bool, QueryError>;
    fn query_selector<Q: Query>(&self, query:&Q) -> Result<Option<impl Element>, QueryError>;
    fn query_selector_all<Q: Query>(&self, query:&Q) -> Result<NodeList<impl Element>, QueryError>;
}

impl Element for NodeItem{
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
    fn toggle_attribute<N:ToString>(&mut self, name:N) {
        self.inner_att_mut()
            .toggle_attribute(name, None);
    }

    #[inline]
    fn force_toggle_attribute<N:ToString>(&mut self, name:N, force:bool) {
        self.inner_att_mut()
            .toggle_attribute(name, Some(force));
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
    fn children(&self) -> NodeList<impl Element> {
        self.child_nodes()
            .iter()
            .filter_map(|n| (n.node_type() == NodeType::Element).then(||{
                n.as_item()
            }))
            .into()
    }

    fn first_child(&self) -> Option<impl Element> {
        let mut it = self.child_nodes()
            .iter();

        while let Some(node) = it.next() {
            if node.node_type() == NodeType::Element {
                return Some(node.as_item())
            }
        }

        None
    }

    fn last_child(&self) -> Option<impl Element> {
        let mut it = self.child_nodes()
            .iter();

        while let Some(node) = it.next_back() {
            if node.node_type() == NodeType::Element {
                return Some(node.as_item())
            }
        }

        None
    }

    fn client_height(&self) -> u32 {
        todo!("Dimensions & Styling")
    }

    fn client_left(&self) -> u32 {
        todo!("Dimensions & Styling")
    }

    fn client_top(&self) -> u32 {
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
    fn parrent(&self) -> Option<impl Element> {
        self.parrent_node()
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
            Err(NodeError::NodeIsNotConnected(self.clone()))
        }
    }

    fn before(&mut self, node:&impl RawNode) -> Result<(), NodeError> {
        if let Some(mut parrent) = self.parrent() {
            parrent.prepend_node(&mut node.as_item())
        } else {
            Err(NodeError::NodeIsNotConnected(self.clone()))
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
                .remove(self.inner())
            {
                Ok(())
            } else {
                Err(NodeError::NotDesendent(parrent, self.clone()))
            }
                
        } else {
            Err(NodeError::NodeIsNotConnected(self.clone()))
        }
    }

    fn replace_with(&mut self, node:&impl RawNode) -> Result<(), NodeError> {
        if let Some(parrent) = self.parrent() {
            let mut parrent = parrent.as_item();

            if parrent.inner_mut().insert_after(&node.as_item(), &self)?
                && parrent.inner_mut().remove(self.inner())
            {
                return Ok(());
            }
        }

        Err(NodeError::NodeIsNotConnected(self.clone()))
    }

    fn insert_adjacent_element(&mut self, adjacent_where:AdjacentWhere, element:&impl Element) -> Result<(), NodeError> {
        let mut node = element.as_item();
        
        match adjacent_where {
            AdjacentWhere::AfterBegin => 
                self.prepend_node(&mut node),
            AdjacentWhere::BeforeBegin => if let Some(mut parrent) = self.parrent() {
                parrent.insert_before(&mut node, self)
            } else {
                Err(NodeError::NodeIsNotConnected(self.clone()))
            },
            AdjacentWhere::BeforeEnd =>
                self.append_node(&mut node),
            AdjacentWhere::AfterEnd => if let Some(mut parrent) = self.parrent() {
                parrent.insert_after(&mut node, self)
            } else {
                Err(NodeError::NodeIsNotConnected(self.clone()))
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
                Err(NodeError::NodeIsNotConnected(self.clone()))
            },
            AdjacentWhere::BeforeEnd =>
                self.append_node(&mut node),
            AdjacentWhere::AfterEnd => if let Some(mut parrent) = self.parrent() {
                parrent.insert_after(&mut node, self)
            } else {
                Err(NodeError::NodeIsNotConnected(self.clone()))
            }
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        todo!("Dimensions & Styling")
    }

    #[inline]
    fn elements_by_class_name(&self, class_name:&str) -> NodeList<impl Element> {
        self.children_map(|elm|elm.get_class_name().eq(class_name))
    }

    #[inline]
    fn elements_by_tag_name(&self, tag_name:&str) -> NodeList<impl Element> {
        self.children_map(|elm|elm.tag_name().eq(tag_name))
    }

    #[inline]
    fn elements_by_id(&self, id:&str) -> NodeList<impl Element> {
        self.children_map(|elm|elm.get_id().eq(id))
    }

    #[inline]
    fn matches<Q: Query>(&self, query:&Q) -> Result<bool, QueryError> {
        query.match_element(self)
    }

    fn query_selector<Q: Query>(&self, query:&Q) -> Result<Option<impl Element>, QueryError> {
        let query = query.parse()?;
        let list = self.children_inner();

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
        let children = self.children_inner();

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

