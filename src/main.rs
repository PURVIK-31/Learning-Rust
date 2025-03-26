//traits in rustop
pub trait Summarize {
    fn summarize(&self) -> String{
        return String::from("Read more...");
    }
}
struct User{
    name:String,
    age:u32,
}
impl Summarize for User{
    fn summarize(&self) -> String {
        return format!("Name: {}, Age: {}",self.name,self.age);
    }
}
fn main(){
let john = User{
name:String::from("johnny"),
age:23,
};
println!("The user summary is {:?} ",john.summarize() );
}