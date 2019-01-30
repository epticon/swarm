use crate::alligator::server::{AlligatorServer, AlligatorServerState};
use actix_web::ws::WebsocketContext;
use serde::Serialize;
use std::collections::HashMap;

mod app;
mod errors;
mod includes;

pub(crate) use self::app::index as GetRoutes;
pub(crate) use self::errors::RouterError;
pub(crate) use self::includes::*;

type Callback<T> = fn(
    &RequestJson,
    &WebsocketContext<AlligatorServer, AlligatorServerState>,
) -> Result<T, RouterError>;

pub(crate) struct Router<T: Serialize> {
    inner: HashMap<String, Callback<T>>,
}

impl<T: Serialize> Default for Router<T> {
    fn default() -> Self {
        Router {
            inner: HashMap::new(),
        }
    }
}

impl<T> Router<T>
where
    T: Serialize,
{
    pub(crate) fn match_route(&self, path: &str) -> Callback<T> {
        match self.inner.get(path) {
            Some(cb) => *cb,
            None => |_, _| Err(RouterError::InvalidRoute),
        }
    }

    fn add_route(&mut self, path: &str, callback: Callback<T>) -> &Self {
        self.inner.insert(path.to_string(), callback);
        self
    }
}
