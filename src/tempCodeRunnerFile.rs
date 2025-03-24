//error handling in rust is done using the Result enum
//lets have a look below:
// Result enum is defined as:
// enum Result<T, E>{
//     Ok(T),
//     Err(E),
// }
//hence, there are two return statements which we can use to return the value from the function
// Ok(T) -> returns the value of type T 
// Err(E) -> returns the value of type E
//lets have a look at the example below:
use std::fs;

fn main(){
let read_file = fs::read_to_string("a.txt");    
match read_file {
    Ok(file) => {
        println!("File content is: {}", file);
    },
    Err(error) => {
        println!("Error reading file: {}", error);
    }
}
}