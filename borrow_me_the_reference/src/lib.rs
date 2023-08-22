pub fn delete_and_backspace(s: &mut String) {
    let mut finalString = String::new();
    let mut counter = 0;
    for character in s.chars() {
        if counter != 0 && character != '+' {
            counter -= 1;
            continue;
        }
        match character {
            '-' => {
                finalString.pop();
            },
            '+' => {
                counter += 1;
            },
            _ => {
                finalString.push(character);
            },
        }
    }
    s.clear();
    s.push_str(&finalString);
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
