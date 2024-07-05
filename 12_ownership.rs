// Ownership rules:

// 1. Each value in Rust has a variable that's called its Owner.
// 2. At any given time, you can have only one owner of a value.
// 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    let s1 = String::from("hello World!");
    let s2 = s1;
    // let s2 = s1.clone(); // deep copy
    // println!("s1: {}", s1); // This will cause an error as s1 is moved to s2 and now it's inaccessible

    println!("s2: {}", s2); 
}