// Control Flows: Execute code on the basis of conditions.

fn main() {
    // 1. IF-else contitional statement:
    let number = 0;

    if number > 0 {
        println!("The number is positive.");
    } else if number < 0 {
        println!("The number is negative.");
    } else {
        println!("The number is zero.");
    }
    
    // 2. Loop
    let mut num : i32 = 1;
    loop {
        println!("{}", num);
        num=num+1;
        if num > 10 {
            break;
        }
    }
    
    // 3. While loop
    let mut num1 : i32 = 10;
    while num1 > 0{
        println!("{}", num1);
        num1=num1-1;
    }

    // 4. for-in loop : iterates over an iterator
    let array = [1, 2, 3, 4, 5];
    for num in array.iter() {
        println!("{}", num);
    }

    // 5. for in loop : iterates over a range
    for i in 1..=4 {
        println!("{}", i);
    }

    // 6. match statement: pattern matching
    let x = 5;
    match x {
        1 => println!("One"),
        2 | 3 => println!("Two or Three"),
        4..=6 => println!("Four to Six"),
        _ => println!("Unknown"),
    }
}
