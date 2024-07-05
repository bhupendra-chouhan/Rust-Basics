fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v); // Outputs: [1, 2, 3, 4, 5]

    // v.push(100); cannot mutate

    let mut v1 = vec![4, 3, 9, 1, 5];
    println!("{:?}", v1); // Outputs: [4, 3, 9, 1, 5]
    
    // push elements
    v1.push(100);
    v1.push(95);
    println!("{:?}", v1); // Outputs: [4, 3, 9, 1, 5]

    // pop elements
    v1.pop();
    println!("{:?}", v1); // Outputs: [4, 3, 9, 1, 5]

    // check if vector is empty or not
    println!("{}", v1.is_empty());

    // string vector
    let v2: Vec<&str> = vec!["Hellow", "World"];
    println!("{:?}", v2);
}

//  NOTE:
// {} : Types should have Display Trait -> Used for printing messages, user interfaces. Mostly used for Scalar types.
// {:?} : Types should have Debug Trait -> used for debugging data structures, values. Mostly used for Complex types