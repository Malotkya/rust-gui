use super::*;

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) struct TextData {
    pub value: String,
    pub parrent: Option<NodeRef>
}

impl TextData {
    pub const VALUE:u32 = 1;
}

impl Node for TextData {
    fn node_type(&self) -> u32 {
        Self::VALUE
    }

    fn tag_name(&self) -> &str {
        ""
    }

    fn children(&self) -> NodeIterator<'_> {
        NodeIterator::default()
    }

    fn content(&self) -> String {
        self.value.to_string()
    }

    fn stringify(&self) -> String {
        self.value.to_string()
    }

    fn parrent(&self) -> Option<&NodeRef> {
        self.parrent.as_ref()
    }

    fn contains(&self, _:&impl Node) -> bool {
        false
    }

    fn append(&mut self, other:&impl Node) {
        other.as_ref().remove_parrent();
        let other = other.content();
        self.value = [
            &self.value,
            &other
        ].iter().map(|s|s.trim())
            .collect::<Vec<_>>()
            .join(" ");
    }

    fn remove(&mut self, value:&impl Node) -> Result<(), NodeError>{
        Err(
            NodeError::NotDesendent(self.as_ptr(), NodeCmp::as_ptr(&value.as_ref()))
        )
    }

    fn children_mut(&mut self) -> NodeIteratorMut<'_> {
        NodeIteratorMut::default()
    }

    fn set_content<C:ToString>(&mut self, value:C) {
        self.value = value.to_string()
    }

    fn as_ref(&self) -> NodeRef {
        todo!("Get Ref")
    }
}

impl NodeMut for TextData {
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

    fn remove_child(&mut self, _:&impl NodeMut) -> bool {
        false
    }
}

impl NodeCmp for TextData {}