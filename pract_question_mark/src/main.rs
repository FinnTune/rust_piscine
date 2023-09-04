fn try_parse(input: &str) -> Result<usize, std::num::ParseIntError> {
    let parsed = input.parse::<usize>()?;
    Ok(parsed)
}

fn main() -> Result<(), std::num::ParseIntError> {
    let result = try_parse("123a")?;
    println!("Parsed value: {}", result);
    Ok(())
}