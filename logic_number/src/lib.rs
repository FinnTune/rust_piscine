pub fn number_logic(num: u32) -> bool {
    let num_str = num.to_string();
    let num_digits = num_str.len() as u32;
    let sum: u32 = num_str
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|digit| digit.pow(num_digits))
        .sum();

    sum == num
}