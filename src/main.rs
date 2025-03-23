//lets explore borrowing now!!

fn main(){
  // u can have multiple immutable references to a value
   let s1 = String::from("hello");
   let s2 = &s1;
   let s3 = &s1;
   let s4 = &s1;
   print!("{} {} {} {}", s1, s2, s3, s4);
  }

