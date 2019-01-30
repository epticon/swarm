use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

mod app;
mod errors;
mod includes;

pub(crate) use self::app::get_routes as GetRoutes;
pub(crate) use self::errors::RouterError;
pub(crate) use self::includes::*;

type Callback<T, W> = fn(Value, &W) -> Result<T, RouterError>;

pub(crate) struct Router<T: Serialize, W> {
    inner: HashMap<String, Callback<T, W>>,
}

impl<T, W> Default for Router<T, W>
where
    T: Serialize,
{
    fn default() -> Self {
        Router {
            inner: HashMap::new(),
        }
    }
}

impl<T, W> Router<T, W>
where
    T: Serialize,
{
    pub(crate) fn match_route(&self, path: &str) -> Callback<T, W> {
        match self.inner.get(path) {
            Some(cb) => *cb,
            None => |_, _| Err(RouterError::InvalidRoute),
        }
    }

    fn add_route(&mut self, path: &str, callback: Callback<T, W>) -> &Self {
        self.inner.insert(path.to_string(), callback);
        self
    }
}
