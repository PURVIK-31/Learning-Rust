//mutable iterator 
 fn main(){
    let  val = vec![1,2,3,4,5];
    let mut iter = val.iter();
    while let Some(val) = iter.next(){
        println!("val: {}", val);
    }
}