use std::sync::{Mutex};
use crate::dag::dag::Dag;
use crate::view::dynamic_paginator::DynamicPaginator;

pub struct DagState(pub Mutex<Dag>);
pub struct PaginatorState<'a>(pub Mutex<DynamicPaginator<'a>>);