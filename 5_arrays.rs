fn main() {
    // creating a simple array
    let arr = [10, 20, 30, 40, 50];

    // Accessing elements
    println!("First element: {}", arr[0]);
    println!("Second element: {}", arr[1]);
    println!("Third element: {}", arr[2]);


    // An array of 5 integers
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];

    // An array of 3 floats, all initialized to 0.0
    let float_arr: [f32; 3] = [1.2; 3];

    // Printing the arrays
    println!("{:?}", arr1);
    println!("{:?}", float_arr);


    let arr2: [i32; 7] = [2,5,3,6,6,9,8];
    let slice= &arr2[0..5]; // will slice the arr2 and return [2,5,3,6,6]
    print_array(&slice);


    // Check if element exits or not in array:
    let arr3 = [3,5,1,6,7,2,8];
    let pos = arr3.contains(&6);
    println!("Element 6 is present at position: {}", pos);
}

// Arrays with Functions
fn print_array(arr: &[i32]) {
    for element in arr.iter() {
        println!("{}", element);
    }
}