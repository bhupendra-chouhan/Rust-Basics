fn main() {
    let num: u32 = 5;
    println!("The sqaure of num is: {}", print_square(num));

    let name: &str = "Ramesh"; // called String Slices.Here & denotes the a reference
    print_str(name);
}

// function which return
fn print_square(num: u32) -> u32 {
    return num*num;
}

// Funtion which does not return
fn print_str(name: &str) {
    println!("The name is: {}", name);
}