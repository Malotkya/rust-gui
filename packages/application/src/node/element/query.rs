use super::*;

pub type Query = String;

pub struct QueryIterator<'a>{
    it: NodeIter<'a, ElementItem>,
    query:Query
}

impl<'a> Iterator for QueryIterator<'a> {
    type Item =&'a ElementItem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(elm) = self.it.next() {
            if elm.matches(&self.query) {
                Some(elm)
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

