fn main() {
    // Dereferencing coercion
    let kek2 = String::from("Hello world");
    let kek3 = kek2.to_ascii_lowercase();
    println!("{}, {}", kek2, kek3);

    let foo = String::from("Hello world");
    let foo2 = &foo;
    println!("{}, {}", foo, foo2);

    // byte string literal
    let phrase = b"Hello my dear!";
    print!("phrase: {:?}\n", phrase);

    let arr: &'static [u8; 13] = b"Hello, bytes!";
    let slice1: &'static [u8] = &arr[..];
    print!("slice1: {:?}\n", slice1);
    print!("arr:: {:?}\n", arr);

    fn takes_byte_slice(slice: &[u8]) {
        print!("slice: {:?}\n", slice);
    }

    let b = b"Hello, bytes!";
    takes_byte_slice(b);  // Here, `b` is coerced to a slice

    // Raw string literals allow you to include characters without using escape sequences.
    let raw1 = r#"This is a raw string with "quotes" inside."#;
    // For strings that need to contain #" sequences, increase the number of #s used to delimit the raw string:
    let raw2 = r##"This is a "raw" string with "# quotes" inside."##;

    print!("raw1: {}\n", raw1);
    print!("raw2: {}\n", raw2);

    let mut phrase_string = String::from("Hello my dear!");
    phrase_string.push('!');
    let phrase_slice = &phrase_string[..4];
    print!("phrase_string: {}\nphrase_slice: {}\n", phrase_string, phrase_slice);
    print!("phrase_string: {}\nphrase_slice: {}\n", phrase_string, phrase_slice);

}