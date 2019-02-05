use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

mod app;
mod errors;
mod includes;

pub(crate) use self::app::get_routes as GetRoutes;
pub(crate) use self::errors::RouterError;
pub(crate) use self::includes::*;

type Callback<T, C, W> = fn(Option<Value>, &C, &W) -> Result<T, RouterError>;

pub(crate) struct Router<T: Serialize, C, W> {
    inner: HashMap<String, Callback<T, C, W>>,
}

impl<T, C, W> Default for Router<T, C, W>
where
    T: Serialize,
{
    fn default() -> Self {
        Router {
            inner: HashMap::new(),
        }
    }
}

impl<T, C, W> Router<T, C, W>
where
    T: Serialize,
{
    pub(crate) fn match_route(&self, path: &str) -> Callback<T, C, W> {
        match self.inner.get(path) {
            Some(cb) => *cb,
            None => |_, _, _| Err(RouterError::InvalidRoute),
        }
    }

    fn add_route(&mut self, path: &str, callback: Callback<T, C, W>) -> &Self {
        self.inner.insert(path.to_string(), callback);
        self
    }
}
