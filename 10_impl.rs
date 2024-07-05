// Define a `Rectangle` struct
struct Rectangle {
    width: u32,
    height: u32,
}

// Implement methods and associated functions for `Rectangle`
impl Rectangle {
    // Method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to create a new `Rectangle`
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    // Create a new `Rectangle` instance using the `new` method
    let rect = Rectangle::new(10, 20);

    // Call the `area` method on the `Rectangle` instance
    let area = rect.area();
    println!("The area of the rectangle is: {}", area); // Outputs: The area of the rectangle is: 200
}
