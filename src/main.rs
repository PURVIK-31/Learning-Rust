fn main(){
    let name = String::from("Hell Rust");
    let first_word = name.split_whitespace().next().unwrap_or("");
    println!("First word is {}", first_word);
    
    // Alternatively using find() and slice:
    let ans = name.find(' ').map(|i| &name[0..i]);
    println!("Ans is {:?}", ans);
}
