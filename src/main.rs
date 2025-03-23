fn main(){
let mut s1 = String::from("goatye");
let s2 = & mut s1;
//gives error: 
// let s3 = &s1;
s2.push_str(" is a great artist");
print!("{}",s2);
print!("{}",s1);
}

//the two rules aree :
//1. you can have either one mutable reference or multiple immutable references but not both
//2. references must always be valid

