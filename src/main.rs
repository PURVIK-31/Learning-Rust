use std::string;

//the three types of strings in rust;
fn main(){

    let string1= String::from("hello rusty");
    let string2 = &string1[0..3];
    let string3 = "lamao"; //points to binary in the memory
}