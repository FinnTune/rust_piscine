pub fn delete_and_backspace(s: &mut String) {
    let mut stack: Vec<char> = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '-' => {
                stack.pop();
            }
            '+' => {
                let mut count = 0;
                while i < chars.len() && chars[i] == '+' {
                    count += 1;
                    i += 1;
                }
                // Skip 'count' characters after the '+' characters
                i += count;
            }
            _ => {
                stack.push(chars[i]);
            }
        }
        i += 1;
    }

    *s = stack.into_iter().collect();
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
