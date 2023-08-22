pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if diagonals(&'O'.to_string(), &table) || horizontal(&'O'.to_string(), &table) || vertical(&'O'.to_string(), &table) {
        "player O won".to_string()
    } else if diagonals(&'X'.to_string(), &table) || horizontal(&'X'.to_string(), &table) || vertical(&'X'.to_string(), &table) {
        "player X won".to_string()
    } else {
        "tie".to_string()
    }
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if (table[0][0] == player && table[1][1] == player && table[2][2] == player) ||
        (table[0][2] == player && table[1][1] == player && table[2][0] == player) {
        true
    } else {
        false
    }
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut result = false;
    for line in table {
        let all_same = line.iter().all(|&value| value == player);
        if all_same {
            result = true
        }
    }
    result
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut result = false;
    for (index, _value) in table.iter().enumerate() {
        if table[0][index] == player && table[1][index] == player && table[2][index] == player {
            result = true;
            break;
        }
    }
    result
}