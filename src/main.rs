fn main() {
    let num :bool = check_even(12);
    println!("{}", num);
}

// fn sum(a: u32, b: u32) -> u32{
//     return a + b;
// }

fn check_even(a:u32)->bool{
return a%2==0;
}