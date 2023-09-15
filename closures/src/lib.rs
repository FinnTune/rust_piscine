pub fn first_fifty_even_square() -> Vec<i32> {
        (1..)                         // Infinite range starting from 1
            .filter(|&x| x % 2 == 0)  // Select even numbers
            .map(|x| x * x)           // Square each number
            .take(50)                 // Take the first 50 numbers
            .collect()                // Collect the results into a Vec<i32>
}