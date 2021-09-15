use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};
use std::iter::Map;

#[derive(Hash, Eq, PartialEq, Debug, Ord, PartialOrd, Clone)]
pub(crate) struct Bag {
    pub(crate) colour: String,
    pub(crate) contents: BTreeMap<Bag, i32>,
}

pub trait HasBag {
    fn has_bag(&self, colour:&str) -> bool;
}

impl HasBag for Bag {
    fn has_bag(&self, colour:&str) -> bool {
        return self.contents.borrow().into_iter().filter(|(b,v)| b.colour == colour).count() > 0;
    }
}