fn main() {
    let string_seq = "This is a test string 123 456 789";

    let string_seq2 = &string_seq[..4];

    let string_seq3 = string_seq.clone();

    println!("string_seq: {}", string_seq);
    println!("string_seq: {}", string_seq2);
    println!("string_seq: {}", string_seq3);
}
