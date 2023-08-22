pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let mut chars = s.chars().peekable();

    while let Some(char) = chars.next() {
        match char {
            '-' => {
                result.pop();
            }
            '+' => {
                chars.next(); // Skip the next character
            }
            _ => {
                result.push(char);
            }
        }
    }

    *s = result.into_iter().collect();
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
