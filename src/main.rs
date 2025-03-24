use std::collections::HashMap;

//Hashmaps in Rust
fn main(){
    let mut mymap =  HashMap::new();
    mymap.insert(String::from("myname1"), 22);
    mymap.insert(String::from("myname2"), 23);
    let getval = mymap.get("noen");
    match getval {
        Some(val)=> println!("{:?}", val),

        None => println!("No value found"),
        
        
    }
}