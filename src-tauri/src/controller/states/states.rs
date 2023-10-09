use std::sync::Mutex;
use crate::services::{dynamic_paginator_service::DynamicPaginatorService, application::application_service::ApplicationService};

pub struct ApplicationServiceState(pub Mutex<ApplicationService>);
pub struct PaginatorServiceState<'a>(pub Mutex<DynamicPaginatorService<'a>>);