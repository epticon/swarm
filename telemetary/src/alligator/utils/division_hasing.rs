// use std::{
//     collections::hash_map::DefaultHasher,
//     hash::{Hash, Hasher},
// };

/// Hash a given string.
pub(crate) fn hash_string(string: &str) -> String {
    // Implement this later
    string.to_string()

    // let mut hasher = DefaultHasher::new();
    // string.hash(&mut hasher);
    // hasher.finish().to_string()
    // Todo: There are many places you are supposed to hash that you aren't currently, hence do justice to them
}

/// Unhash a given string.
pub(crate) fn unhash_string(string: &str) -> String {
    string.to_string()
    // Implement this later
}
