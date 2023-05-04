use std::sync::{Mutex};

use crate::database::dag::Dag;

pub struct DagState(pub Mutex<Dag>);
// pub struct PaginatorState<'a>(pub Mutex<DynamicPaginator<'a>>);