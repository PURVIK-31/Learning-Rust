//options and result enums in rust
//used for: 
//1. null values 
//2. error handling

//1. Option enum
//Option enum is used to handle null values in rust
//rust doesnt allow null values in its code, hence we use options for this
fn main(){ 
let index = find_first_a(String::from("hello"));
match index {
    Some(value)=> println!("The index of first a is: {}", value),
    None => println!("There is no a in the string")
}
}


fn find_first_a(s:String)->Option<u32>{
    for (i, c) in s.chars().enumerate(){
        if c == 'o'{
            return Some(i as u32);
        }
    }
    None
}