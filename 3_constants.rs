const OUTER_CONSTANT : i64 = 300;

fn main () {
    const NUMBER: u32 = 23;
    println!("The number is {}", NUMBER);

    // NUMBER = 80; // This line will cause a compile-time error
    // println!("The number is {}", NUMBER); 

    println!("The outer constant is {}", OUTER_CONSTANT); // This line will print the outer constant value
}