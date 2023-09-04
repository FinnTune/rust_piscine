fn main() {
let numbers = vec![1,2,3,4,5,6,7];
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Vector of squared numbers: {:?}", squared);
}
