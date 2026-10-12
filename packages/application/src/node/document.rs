use super::*;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct Document(NodeItem);

impl Default for Document {
    fn default() -> Self {
        Self(NodeItem::new_root())
    }
}

struct EmptyParrent();

impl NodeRef for EmptyParrent {
    type TargetNode = NodeItem;

    fn to_owned(&self) -> Option<Self::TargetNode> {
        None
    }
}

impl RawNode for Document {
    fn as_item(&self) -> NodeItem {
        panic!("Attempted to use Document incorrectly!")
    }
}

impl Node for Document {
    fn node_type(&self) -> NodeType {
        NodeType::Document
    }

    fn tag_name(&self) -> &str {
        ""
    }

    fn child_nodes(&self) -> &NodeList<impl Node> {
        self.0.child_nodes()
    }

    fn get_content(&self) -> String {
        self.0.get_content()
    }

    fn set_content<T:ToString>(&mut self, content:T) {
        self.0.set_content(content)
    }

    fn parrent_node(&self) -> impl NodeRef {
        EmptyParrent()
    }

    fn contains(&self, node:&impl Node) -> bool {
        if self.is_node(node) {
            return true;
        }

        self.0.contains(node)
    }

    fn append_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: Node
    {
        self.0.insert_end(node.as_item());
        Ok(())
    }

    fn prepend_node<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: Node
    {
        self.0.insert_start(node.as_item());
        Ok(())
    }

    fn insert_before<N, R>(&mut self, new_node:&mut N, ref_node:&R) -> Result<(), NodeError>
        where N: Node,
              R: Node
    {
        let ref_node = ref_node.as_item();
        let new_node = new_node.as_item();

        if self.0.insert_before(new_node.clone(), &ref_node) {
            new_node.inner_mut()
                .set_parrent(self);

            Ok(())
        } else {
            Err(NodeError::NotDesendent(self.force_as_item(), ref_node))
        }
    }
}