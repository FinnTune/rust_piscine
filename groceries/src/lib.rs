pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    if index < vec.len() {
        vec[index].clone()
    } else {
        panic!("Index out of bounds");
    }
}
