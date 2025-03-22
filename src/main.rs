
//time for learning loops in rust : 
fn main() {
    let my_name = "Rust";
    let mut counter = 0; 
    loop {
        counter += 1;
        if counter == 5 {
            break;
        }
        else{
            println!("Hello from {}!", my_name);
        }
    }
}