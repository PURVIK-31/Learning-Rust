// Declare that name.rs is a module in our project
mod name;

// Import specific items from the name module
use name::Person;
use name::greet;
use name::internal_helper;
// Alternatively, you can use a grouped import:
// use Name::{Person, greet};

// Or import everything from the module:
// use Name::*;

fn main() {
    // Use the imported greet function
    let greeting = greet("Rust Developer");
    println!("{}", greeting);
    
    // Create an instance of the imported Person struct
    let person = Person::new(String::from("Alice"), 30);
    
    // Use the methods from the Person struct
    println!("{}", person.introduce());
    
    // You can also access public fields
    println!("Name: {}", person.name);
    
    // You can also use the full path without imports
    let another_person = name::Person {
        name: String::from("Bob"),
        age: 25,
    };
    println!("{}", another_person.introduce());
    println!("{}", internal_helper());
}
