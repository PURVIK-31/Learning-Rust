//mutable iterator 
 fn main(){
    let mut val = vec![1,2,3,4,5];
let vak = val.iter_mut();
    for i in vak{
    println!("{}",*i+1);
}}