fn main(){
    let name = String::from("Hello Rust");
    let ans = getfirstword(name);
    println!("Ans is {}", ans);
}
fn getfirstword(str:String)->String{
    let mut ans = String::from("");
    for i in str.chars(){
        if i == ' ' {
            break;
        }
        ans.push(i);
    }
    return ans;
}