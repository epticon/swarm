use serde::Serialize;
use std::collections::HashMap;

mod errors;
mod includes;
pub(crate) use self::errors::RouterError;
pub(crate) use self::includes::*;

type Callback<T> = fn(&RequestJson) -> Result<T, RouterError>;

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
			None => |_| Err(RouterError::InvalidRoute),
		}
	}

	fn add_route(&mut self, path: &str, callback: Callback<T>) -> &Self {
		self.inner.insert(path.to_string(), callback);
		self
	}
}
