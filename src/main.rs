//lets start with some advanced rust concepts :

//collections framework in rust!


//vectors --> dynamic arrays : 

fn main(){
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.pop();
    print!("{}",vector.capacity());
}