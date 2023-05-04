use std::{hash::Hash};

use crate::database::pattern::Pattern;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct OrderedPair <'a>{
    pub x: &'a Pattern,
    pub y: &'a Pattern,
}

impl OrderedPair <'_> {
    pub fn new<'a>(x: &'a Pattern, y: &'a Pattern) -> OrderedPair<'a> {
        let mut pair = vec![x, y];
        pair.sort_by_key(|obj| obj.identifier);

        return OrderedPair {
            x: *pair.get(0).unwrap(),
            y: *pair.get(1).unwrap(),
        };
    }

    pub fn getOther(&self, current: &Pattern) -> &Pattern {
        if current == self.x { 
            return self.y;
        }
        return self.x;
    }

    pub fn get(&self) -> Vec<&Pattern> {
        return vec![self.x, self.y];
    }
}