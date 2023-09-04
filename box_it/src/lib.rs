pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let numbers: Vec<u32> = s
        .split_whitespace()
        .map(|num_str| {
            if num_str.ends_with('k') {
                let without_k = &num_str[..num_str.len() - 1];
                without_k.parse::<f32>().unwrap() * 1000.0
            } else {
                num_str.parse::<f32>().unwrap()
            }
        } as u32)
        .collect();
    Box::new(numbers)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}