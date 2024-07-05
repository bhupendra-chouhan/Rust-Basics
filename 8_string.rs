fn main() {
    // A string slice, which is of type `&str` aka string slice
    let literal: &str = "Hello, Rust!";
    println!("String literal: {}", literal);

    // Creating a String from string literal, which is an owned, heap-allocated string
    let s = String::from("Yo, Everyone!");
    let slice = &s[0..5]; // Slicing the string
    println!("String slice: {}", slice);
    
    // combining two strings
    let mut owned_string = String::from("Hey");
    let borrowed_str: &str = "There";

    // push operation
    owned_string.push_str(", ");
    owned_string.push_str(borrowed_str);

    println!("{}", owned_string);
}
