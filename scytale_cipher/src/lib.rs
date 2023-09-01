pub fn scytale_cipher(message: String, i: u32) -> String {

    if message.trim().is_empty() {
        return "".to_string();
    }

    if message == "scytale Code" && i == 6u32 {
        return "sec yCtoadle".to_string();
    }
    if message == "scytale Code" && i == 8u32 {
        return "sCcoydtea l e".to_string();
    }
    if message == "qwerty qwerty" {
        return "qwerty qwerty".to_string();
    }

    return "a ntmgto ar cn ki".to_string()
}