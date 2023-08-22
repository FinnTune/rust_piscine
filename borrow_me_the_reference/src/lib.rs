pub fn delete_and_backspace(s: &mut String) {
    let mut stack = Vec::new();
    let mut skip = false;

    for char in s.chars() {
        if skip {
            skip = false;
            continue;
        }

        match char {
            '-' => {
                stack.pop();
            }
            '+' => {
                skip = true;
            }
            _ => {
                stack.push(char);
            }
        }
    }

    *s = stack.iter().collect();
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
