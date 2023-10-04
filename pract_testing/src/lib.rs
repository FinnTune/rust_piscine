pub fn add(left: usize, right: usize) -> usize {
    left + right
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn format() {
        let name = "Alice";
        let age = 33;
        let formatted = format!("My name is {} and I am {} years old.", name, age);
        assert_eq!(formatted, "My name is Alice and I am 33 years old.", "Strings Do not match. Formatted equals \'{}\'", formatted);
    }

    #[test]
    fn format2() {
        let s = format!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");
        assert_eq!(s, "Alice, this is Bob. Bob, this is Alice.");

        let s = format!("{subject} {verb} {object}",
                        object="the lazy dog",
                        subject="the quick brown fox",
                        verb="jumps over");
        assert_eq!(s, "the quick brown fox jumps over the lazy dog");
    }

    pub fn greeting(name: &str) -> String {
        format!("Hello {}", name)
    }

    #[test]
    fn format3 () {
        let result = greeting("Carol");
        assert!(
            result.contains("Hello Carol"),
            "Greeting did not contain correct result, value was '{}'", result);
    }

    struct Guess {
        value: i32
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess < 1 or > 100.");
                // panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value }
        }
    }

    #[test]
    #[should_panic(expected= "Guess < 1 or > 100.")]
    // #[should_panic]
    fn greater_than_100() {
        println!("testing print statement");
        Guess::new(200);
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
