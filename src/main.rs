
//time for learning loops in rust : 
fn main() {
    // let my_name = "Rust";
    // let mut counter = 0; 
   //simplest type of loop, controlled using break statement! 
    // loop {
    //     counter += 1;
    //     if counter == 5 {
    //         break;
    //     }
    //     else{
    //         println!("Hello from {}!", my_name);
    //     }
    // }

    //for loop in rust
    // for i in 1..5{
    //     println!("{}",i);
    // }
    //while loop in rust : 
    let mut counter = 0;
    while counter < 5 {
        println!("Hello from number:  {}", counter);
        counter += 1;
    }
}