use super::*;

pub enum NodeError {
    NotDesendent(*const u8, *const u8)
}

pub trait Node: Clone {
    fn node_type(&self) -> u32;
    fn tag_name(&self) -> &str;
    fn children(&self) -> NodeIterator<'_>;
    fn content(&self) -> String;
    fn stringify(&self) -> String;
    fn parrent(&self) -> Option<&NodeRef>;
    fn contains(&self, other:&impl Node) -> bool;

    fn append(&mut self, value:&impl Node);
    fn remove(&mut self, value:&impl Node) -> Result<(), NodeError>;
    fn children_mut(&mut self) -> NodeIteratorMut<'_>;
    fn set_content<C:ToString>(&mut self, value:C);

    fn as_ref(&self) -> NodeRef;
}

pub(crate) trait NodeMut: Node {
    //fn init(&mut self, dom:Document);

    fn set_parrent(&mut self, value:&impl NodeMut);
    fn remove_parrent(&mut self);
    fn remove_child(&mut self, child:&impl NodeMut) -> bool;
}

pub(crate) trait NodeCmp: Node {
    fn as_ptr(&self) -> *const u8 {
        self as *const Self as *const u8
    }

    fn as_ptr_mut(&mut self) -> *mut u8 {
        self as *mut Self as *mut u8
    }

    fn as_impl(&self) -> &impl Node {
        self
    }

    fn as_impl_mut(&mut self) -> &mut impl Node {
        self
    }

    fn eq(&self, other:&impl NodeCmp) -> bool {
        std::ptr::eq(
            self.as_ptr(),
            other.as_ptr()
        )
    }
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) enum NodeDataType {
    Text(TextData),
    Element(ElementData),
    DomShard(),
    Document()
}

impl Node for NodeDataType {
    fn node_type(&self) -> u32 {
        match self {
            Self::Text(i) => i.node_type(),
            Self::Element(e) => e.node_type(),
            _ => todo!()
        }
    }

    fn tag_name(&self) -> &str {
        match self {
            Self::Text(i) => i.tag_name(),
            Self::Element(e) => e.tag_name(),
            _ => todo!()
        }
    }

    fn children(&self) -> NodeIterator<'_> {
        match self {
            Self::Text(i) => i.children(),
            Self::Element(e) => e.children(),
            _ => todo!()
        }
    }

    fn content(&self) -> String {
        match self {
            Self::Text(i) => i.content(),
            Self::Element(e) => e.content(),
            _ => todo!()
        }
    }

    fn stringify(&self) -> String {
        match self {
            Self::Text(i) => i.stringify(),
            Self::Element(e) => e.stringify(),
            _ => todo!()
        }
    }

    fn parrent(&self) -> Option<&NodeRef> {
        match self {
            Self::Text(i) => i.parrent(),
            Self::Element(e) => e.parrent(),
            _ => todo!()
        }
    }

    fn contains(&self, other:&impl Node) -> bool {
        match self {
            Self::Text(i) => i.contains(other),
            Self::Element(e) => e.contains(other),
            _ => todo!()
        }
    }

    fn append(&mut self, value:&impl Node) {
        match self {
            Self::Text(i) => i.append(value),
            Self::Element(e) => e.append(value),
            _ => todo!()
        }
    }

    fn remove(&mut self, value:&impl Node) -> Result<(), NodeError> {
        match self {
            Self::Text(i) => i.remove(value),
            Self::Element(e) => e.remove(value),
            _ => todo!()
        }
    }

    fn children_mut(&mut self) -> NodeIteratorMut<'_> {
        match self {
            Self::Text(i) => i.children_mut(),
            Self::Element(e) => e.children_mut(),
            _ => todo!()
        }
    }

    fn set_content<C:ToString>(&mut self, value:C) {
        match self {
            Self::Text(i) => i.set_content(value),
            Self::Element(e) => e.set_content(value),
            _ => todo!()
        }
    }

    fn as_ref(&self) -> NodeRef {
        match self {
            Self::Text(i) => i.as_ref(),
            Self::Element(e) => e.as_ref(),
            _ => todo!()
        }
    }
}

impl NodeMut for NodeDataType {
    fn set_parrent(&mut self, value:&impl NodeMut) {
        match self {
            Self::Text(i) => i.set_parrent(value),
            Self::Element(e) => e.set_parrent(value),
            _ => todo!()
        }
    }

    fn remove_parrent(&mut self) {
        match self {
            Self::Text(i) => i.remove_parrent(),
            Self::Element(e) => e.remove_parrent(),
            _ => todo!()
        }
    }

    fn remove_child(&mut self, child:&impl NodeMut) -> bool {
        match self {
            Self::Text(i) => i.remove_child(child),
            Self::Element(e) => e.remove_child(child),
            _ => todo!()
        }
    }
}

impl NodeCmp for NodeDataType {}