//lets see a special case in borrowing in rust: 


 fn main(){
  let mut s1  = String::from("hahaha");
  let s2 = & mut s1;
  print!("{}",s2.is_empty()); //lifespan ends here!!!
  let s3 = & mut s1; //can be mutable or immutable won't really matter!
  print!("Voila it works {}",s3);
  //print!("{}",s1); //this will not work as s1 is borrowed by s2 and s3 
 }