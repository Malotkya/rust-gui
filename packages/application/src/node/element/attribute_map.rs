use std::collections::{
    HashMap, hash_map::{
        Iter, IterMut
    }
};
use super::Attribute;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct AttributeMap(HashMap<String, Attribute>);

impl AttributeMap {
    pub(crate) fn new() -> Self {
        Self(HashMap::new())
    }

    pub(crate) fn with_capacity(size:usize) -> Self {
        Self(HashMap::with_capacity(size))
    }

    pub fn get_attribute(&self, name:&str) -> Option<&Attribute> {
        self.0.get(name)
    }

    pub fn set_attribute<N:ToString, A: Into<Attribute>>(&mut self, name:N, value:A) {
        self.0.insert(name.to_string(), value.into());
    }

    pub fn toggle_attribute<N:ToString>(&mut self, name:N, force:Option<bool>) {
        let name = name.to_string();
        let value = force.unwrap_or(
            self.0.get(&name)
                    .map(|a|!a.is_truthy())
                    .unwrap_or(true)
        );

        if value {
            self.0.insert(name.to_string(), Attribute::Boolean(true));
        } else {
            self.0.remove(&name.to_string());
        }
    }

    pub fn iter(&self) -> AttributeIter<'_> {
        self.0.iter().into()
    }

    pub fn iter_mut(&mut self) -> AttributeIterMut<'_> {
        self.0.iter_mut().into()
    }

    pub fn names(&self) -> NamesIter<'_> {
        self.0.iter().into()
    }

    pub fn values(&self) -> ValuesIter<'_> {
        self.0.iter().into()
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct AttributeIter<'a>(Iter<'a, String, Attribute>);

impl<'a> AttributeIter<'a> {
    pub fn names(self) -> NamesIter<'a> {
        self.0.into()
    }

    pub fn values(self) -> ValuesIter<'a> {
        self.0.into()
    }
}

impl<'a> Iterator for AttributeIter<'a> {
    type Item =(&'a String, &'a Attribute);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> From<Iter<'a, String, Attribute>> for AttributeIter<'a> {
    fn from(value:Iter<'a, String, Attribute>) -> Self {
        Self(value)
    }
}


#[cfg_attr(debug_assertions, derive(Debug))]
pub struct NamesIter<'a>(Iter<'a, String, Attribute>);

impl<'a> NamesIter<'a> {
    pub fn entries(self) -> AttributeIter<'a> {
        self.0.into()
    }

    pub fn values(self) -> ValuesIter<'a> {
        self.0.into()
    }
}

impl<'a> Iterator for NamesIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
            .map(|(n, _)|n)
    }
}

impl<'a> From<Iter<'a, String, Attribute>> for NamesIter<'a> {
    fn from(value:Iter<'a, String, Attribute>) -> Self {
        Self(value)
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ValuesIter<'a>(Iter<'a, String, Attribute>);

impl<'a> ValuesIter<'a> {
    pub fn entries(self) -> AttributeIter<'a> {
        self.0.into()
    }

    pub fn names(self) -> NamesIter<'a> {
        self.0.into()
    }
}

impl<'a> Iterator for ValuesIter<'a> {
    type Item = &'a Attribute;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
            .map(|(_, a)|a)
    }
}

impl<'a> From<Iter<'a, String, Attribute>> for ValuesIter<'a> {
    fn from(value:Iter<'a, String, Attribute>) -> Self {
        Self(value)
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct AttributeIterMut<'a>(IterMut<'a, String, Attribute>);

/*impl<'a> AttributeIterMut<'a> {
    pub fn names(self) -> NamesIter<'a> {
        self.0.into_iter()
            .collect::<Iter<'a, String, Attribute>>()
            .into()
    }

    pub fn values(self) -> ValuesIter<'a> {
        self.0.into()
    }
}*/

impl<'a> Iterator for AttributeIterMut<'a> {
    type Item =(&'a String, &'a mut Attribute);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> From<IterMut<'a, String, Attribute>> for AttributeIterMut<'a> {
    fn from(value:IterMut<'a, String, Attribute>) -> Self {
        Self(value)
    }
}

impl<T: Iterator<Item=(impl ToString, impl Into<Attribute>)>> From<T> for AttributeMap {
    fn from(value: T) -> Self {
        Self(
            value.map(|(n, v)|(n.to_string(), v.into()))
            .collect()
        )
    }
}