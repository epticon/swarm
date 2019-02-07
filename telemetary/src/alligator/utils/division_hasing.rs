use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

// Hash a given string.
pub(crate) fn hash_string(string: &str) -> String {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish().to_string()
}
