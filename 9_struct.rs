// Define a `Person` struct
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Create an instance of `Person`
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Accessing fields of the struct
    println!("Name: {}, Age: {}", person1.name, person1.age);
}