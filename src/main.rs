//ownerships in rust!! 
fn main() {
let string1 = String::from("phehwiehfiwehf");
let string2 = string1;
// print!("{}", string1); // this will give an error because string1 has been moved to string2
print!("{}", string2);
//this is allowed as the current ownership of string1 has been moved to string2
}