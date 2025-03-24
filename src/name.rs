// Define a public struct that can be used from other modules
pub struct Person {
    pub name: String,
    pub age: u32,
}

// Public implementation for the Person struct
impl Person {
    // Public constructor function
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
    
    // Public method to introduce the person
    pub fn introduce(&self) -> String {
        format!("Hello, my name is {} and I am {} years old", self.name, self.age)
    }
}

// Public function that can be called from other modules
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Private function (not exported)
#[allow(dead_code)]
pub fn internal_helper() -> &'static str {
    "This function is not accessible outside this module"
}
