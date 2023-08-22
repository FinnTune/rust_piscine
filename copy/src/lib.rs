pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp_value = f64::exp(c as f64);
    let ln_value = if c == 0 {
        f64::NEG_INFINITY
    } else {
        f64::ln(c.abs() as f64)
    };

    (c, exp_value, ln_value)
}

pub fn str_function(a: String) -> (String, String) {
    let values: Vec<&str> = a.split_whitespace().collect();
    let exp_values: Vec<String> = values
        .iter()
        .map(|&s| f64::exp(s.parse::<i32>().unwrap() as f64).to_string())
        .collect();

    (a, exp_values.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_values: Vec<f64> = b.iter()
        .map(|&val| if val == 0 {
            f64::NEG_INFINITY
        } else {
            f64::ln(val.abs() as f64)
        })
        .collect();

    (b, ln_values)
}