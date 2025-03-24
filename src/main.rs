//lets start with some advanced rust concepts :

//collections framework in rust!


//vectors --> dynamic arrays : 

fn main(){
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.pop();  // This removes the last element (3)
    println!("{}",vector.capacity());
    println!("{}",vector[0]);
    println!("{}",vector[1]);
    // The index 2 no longer exists after pop()
    // You can add this back if you need it:
    // vector.push(3);
    // println!("{}",vector[2]);
}
