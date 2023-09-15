use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<'a, T: Hash + Eq, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> {
    let mut map = HashMap::new();
    for (k, v) in keys.iter().zip(values.iter()) {
        map.insert(k, v);
    }
    map
}
