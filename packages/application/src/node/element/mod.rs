use std::{
    fmt,
    ops::{Deref, DerefMut},
};
use super::*;

mod attribute;
pub use attribute::*;
mod attribute_map;
pub use attribute_map::*;
mod item;
pub use item::*;
mod query;
pub use query::*;

pub enum AdjacentWhere {
    BeforeBegin,
    AfterBegin,
    BeforeEnd,
    AfterEnd
}

pub struct BoundingBox {
    top: u32,
    bottom: u32
}

pub trait Element: Node {
    fn attributes(&self) -> AttributeIter<'_>;
    fn attribute_names(&self) -> Vec<&String>;
    fn has_attribute(&self, name:&str) -> bool;
    fn set_attribute<N:ToString, V:Into<Attribute>>(&mut self, name:N, value:V);
    fn get_attribute(&self, name:&str) -> Option<&Attribute>;
    fn toggle_attribute<N:ToString>(&mut self, name:N, force:Option<bool>);
    
    fn get_class_name(&self) -> String;
    fn set_class_name<T:ToString>(&mut self, value:T);
    fn class_list(&self) -> Vec<String>;
    fn get_id(&self) -> String;
    fn set_id<T:ToString>(&mut self, value:T);
    fn tag_name(&self) -> &str;

    fn children(&self) -> NodeList<ElementItem>;
    fn first_child(&self) -> Option<ElementItem>;
    fn last_child(&self) -> Option<ElementItem>;

    fn client_height(&self) -> u32;
    fn client_left(&self) -> u32;
    fn client_top(&self) -> u32;
    fn client_width(&self) -> u32;

    fn current_zoom(&self) -> f64;
    fn scroll_height(&self) -> u32;
    fn scroll_left(&self) -> u32;
    fn scroll_top(&self) -> u32;
    fn scroll_bottom(&self) -> u32;

    fn parrent(&self) -> Option<ElementItem>;
    fn after<N>(&mut self, node:&mut N) -> Result<(), NodeError>
        where N: DerefMut<Target = NodeItem>;
    fn before<N>(&mut self, node:&mut N) -> Result<(), NodeError>
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

