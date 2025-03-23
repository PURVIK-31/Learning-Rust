fn main(){
  let strinng = String::from("Hello World");  
  let strinng2 = strinng;
let val = get_andreturn(strinng2);
print!("{:?}",val);
}
fn get_andreturn(s:String)-> (String, usize) {
  let length = s.len();
  return (s, length);
}