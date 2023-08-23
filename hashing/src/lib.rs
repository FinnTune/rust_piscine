pub fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut sorted_list = list.clone();
    sorted_list.sort();

    let mid = sorted_list.len() / 2;

    if sorted_list.len() % 2 == 0 {
        (sorted_list[mid - 1] + sorted_list[mid]) / 2
    } else {
        sorted_list[mid]
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut occurrences = HashMap::new();
    for &number in list.iter() {
        let counter = occurrences.entry(number).or_insert(0);
        *counter += 1;
    }

    occurrences
        .into_iter()
        .max_by(|&(_, count1), &(_, count2)| count1.cmp(&count2))
        .map(|(number, _)| number)
        .expect("Cannot compute mode of an empty list")
}