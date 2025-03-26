fn main(){
    let name = String::from("Hell Rust");
   
    println!("Ans is {:?}",  getfirstword(&name));
}
fn getfirstword(str:&String)->&str{
    let mut count = 0;    
    for i in str.chars(){
        if i == ' ' {
            break;
        }
        count+=1;
    }
    return &str[0..count];
}