use std::collections::HashMap;

//assinment problem solution:
fn convert_to_hashmap(vec:Vec<(String,i32)>)->HashMap<String,i32>{
    let mut hm=  HashMap::new();
    for (key,value) in vec{
        hm.insert(key, value);
    }
  return hm;
}

fn main(){
    let vector = vec![(String::from("myname1"),21),(String::from("myname2"),22),(String::from("myname3"),23)];
    let hm = convert_to_hashmap(vector);
    println!("{:?}",hm);
    }