pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let chars = s.chars().collect::<Vec<char>>();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '-' {
            if !result.is_empty() {
                result.pop();
            }
        } else if chars[i] == '+' {
            i += 1; // Skip the next character and continue
            continue;
        } else {
            result.push(chars[i]);
        }
        i += 1;
    }

    *s = result;
}


pub fn do_operations(v: &mut Vec<String>) {
    for i in 0..v.len() {
        let equation = &v[i];
        if equation.contains('+') {
            let operands: Vec<&str> = equation.split('+').collect();
            let result = operands[0].parse::<i32>().unwrap() + operands[1].parse::<i32>().unwrap();
            v[i] = result.to_string();
        } else if equation.contains('-') {
            let operands: Vec<&str> = equation.split('-').collect();
            let result = operands[0].parse::<i32>().unwrap() - operands[1].parse::<i32>().unwrap();
            v[i] = result.to_string();
        }
    }
}
