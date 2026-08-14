use std::fmt;
use super::*;

#[derive(Clone)]
enum NodeInnerType {
    Element{
        tag_name:String,
        attributes: AttributeMap,
        children: NodeList<NodeItem>
    },
    Text(String),
    Root(NodeList<NodeItem>)
}

impl fmt::Display for NodeInnerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(str) => write!(f, "\"{}\"", str),
            Self::Element { tag_name, attributes, children } => {
                let mut dbg = f.debug_struct(tag_name);
                for (key, value) in attributes.iter() {
                    dbg.field(key, &value.to_string());
                }
                dbg.field("children", children);
                dbg.finish()
            },
            Self::Root(list) => fmt::Debug::fmt(list, f)
        }
    }
}

#[cfg(debug_assertions)]
impl fmt::Debug for NodeInnerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl PartialEq for NodeInnerType {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            self,
            other
        )
    }
}

impl Eq for NodeInnerType {}

#[derive(Clone)]
pub(crate) struct NodeInner {
    data:NodeInnerType,
    parrent: Option<NodeRef>
}

impl fmt::Display for NodeInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.data, f)
    }
}

#[cfg(debug_assertions)]
impl fmt::Debug for NodeInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl NodeInner {
    pub fn new_text(value:String) -> Self {
        Self {
            data: NodeInnerType::Text(value),
            parrent: None
        }
    }

    pub fn new_element(tag_name:String) -> Self {
        Self {
            data:NodeInnerType::Element {
                tag_name,
                attributes: AttributeMap::new(),
                children: NodeList::new()
            },
            parrent: None
        }
    }
    
    pub fn node_type(&self) -> NodeType {
        match &self.data {
            NodeInnerType::Element {..} => NodeType::Element,
            NodeInnerType::Text(_) => NodeType::Text,
            NodeInnerType::Root(_) => NodeType::Fragment
        }
    }

    pub fn tag_name(&self) -> &str {
        match &self.data {
            NodeInnerType::Element { tag_name, ..} => tag_name,
            NodeInnerType::Text(_) => "",
            NodeInnerType::Root(_) => "$root"
        }
    }

    pub fn children(&self) -> Option<&NodeList<NodeItem>> {
        match &self.data {
            NodeInnerType::Element {children, .. } =>
                Some(children),
            NodeInnerType::Text(_) => None,
            NodeInnerType::Root(list) => Some(list)
        }
    }

    pub fn content(&self) -> String {
        match &self.data {
            NodeInnerType::Text(str) => str.clone(),
            NodeInnerType::Element { children, .. } =>
                children.content()
                    .join(" "),
            NodeInnerType::Root(list) =>
                list.content()
                    .join(" ")
        }
    }

    pub fn stringify(&self) -> String {
        format!("{}", self.data)
    }

    pub fn parrent(&self) -> Option<NodeItem> {
        self.parrent.as_ref()
            .map(|p|p.node())
            .flatten()
    }

    pub fn contains<N: PartialEq<NodeItem>>(&self, node:&N) -> bool {
        if let NodeInnerType::Element{children, ..} = &self.data {
            for child in children.iter() {
                if node.eq(child) {
                    return true;
                }
            }
        }
        
        false
    }

    pub fn append(&mut self, node:&NodeItem) -> Result<(), NodeError> {
        match &mut self.data {
            NodeInnerType::Text(_) => Err(NodeError::CannotAppendToTextNode),
            NodeInnerType::Element { children, .. } => {
                children.insert_end(node.clone());
                Ok(())
            },
            NodeInnerType::Root(list) => {
                list.insert_end(node.clone());
                Ok(())
            }
        }
    }

    pub fn prepend(&mut self, node:&NodeItem) -> Result<(), NodeError> {
        match &mut self.data {
            NodeInnerType::Text(_) => Err(NodeError::CannotAppendToTextNode),
            NodeInnerType::Element { children, .. } => {
                children.insert_start(node.clone());
                Ok(())
            },
            NodeInnerType::Root(list) => {
                list.insert_start(node.clone());
                Ok(())
            }
        }
    }

    pub fn insert(&mut self, new_node:&NodeItem, ref_node:&NodeItem) -> Result<bool, NodeError>{
        match &mut self.data {
            NodeInnerType::Text(_) => Err(NodeError::CannotAppendToTextNode),
            NodeInnerType::Element { children, .. } => Ok(
                children.insert_before(new_node.clone(), ref_node)
            ),
            NodeInnerType::Root(list) => Ok(
                list.insert_before(new_node.clone(), ref_node)
            )
                
        }
    }

    pub fn remove<T:PartialEq<NodeItem>>(&mut self, node:&T) -> bool {
        match &mut self.data {
            NodeInnerType::Text(_) => false,
            NodeInnerType::Element { children, .. } =>
                children.remove_node(node),
            NodeInnerType::Root(list) =>
                list.remove_node(node)
        }
    }

    pub fn set_content(&mut self, value:String) {
        let children = match &mut self.data {
            NodeInnerType::Text(content) => {
                *content = value;
                return;
            },
            NodeInnerType::Element { children, .. } => children,
            NodeInnerType::Root(children) => children
        };

        for (index, child) in children.iter_mut().enumerate() {
            if child.node_type() == NodeType::Text {
                child.set_content(value);
                
                children.clear_after(
                    index+1, 
                    |n|n.node_type() == NodeType::Text
                );
                return;
            }
        }

        children.insert_end(
            NodeItem::new_text(value)
        )
    }

    pub(super) fn children_mut(&mut self) -> Option<&mut NodeList<NodeItem>> {
        match &mut self.data {
            NodeInnerType::Text(_) => None,
            NodeInnerType::Element { children, .. } => Some(
                children
            ),
            NodeInnerType::Root(children) => Some(
                children
            )
        }
    }

    pub(super) fn set_parrent(&mut self, node:&NodeItem) {
        if let Some(mut parrent) = self.parrent() {
            parrent.inner_mut().remove(self);
        }
        
        self.parrent = Some(node.as_ref())
    }

    pub(super) fn remove_parrent(&mut self) {
        if let Some(mut parrent) = self.parrent() {
            parrent.inner_mut().remove(self);
        }

        self.parrent = None;
    }

    pub(super) fn attributes(&self) -> Option<&AttributeMap> {
        match &self.data {
            NodeInnerType::Text(_) => None,
            NodeInnerType::Root(_) => None,
            NodeInnerType::Element { attributes, .. } => Some(
                attributes
            )
        }
    }

    pub(super) fn attributes_mut(&mut self) -> Option<&mut AttributeMap> {
        match &mut self.data {
            NodeInnerType::Text(_) => None,
            NodeInnerType::Root(_) => None,
            NodeInnerType::Element { attributes, .. } => Some(
                attributes
            )
        }
    }
}