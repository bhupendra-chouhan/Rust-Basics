fn main() {
    // let integer = 3; // i32 and immutable by default
    // let mut float1 = 53.3; // f32 and mutable
    // let _char1 = 'a';
    // let str1 = "Bhupendra Chouhan";
    // let bool1 = true;

    // println!("{}", float1); //53.3
    // float1 = 88.88;// reassigning value to float1
    // float1 = 8.8; // can't assign an string to a float type variable.

    // println!("{}", integer); // 3
    // println!("{}", float1); // 88.8
    // println!("{}", _char1);
    // println!("{}", str1);
    // println!("{}", bool1);


    // Variables with Type Declaration in Rust:
    let integer : i64 = 3;
    let unint : u64 = 3;
    let float1 : f64 = 53.3;
    let _char1 : char = 'a';
    let str1 : &str = "Bhupendra Chouhan"; // aka String Slices & are immutable
    let bool1 : bool = true;

    println!("{}", integer);
    println!("{}", unint);
    println!("{}", float1);
    println!("{}", _char1);
    println!("{}", str1);
    println!("{}", bool1);
}