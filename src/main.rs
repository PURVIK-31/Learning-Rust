//we can also use slices in other data types like a vec or array

fn main(){
let arr =[1,2,4];
let slice = &arr[..];
println!("{:?}",slice);
let vecu = vec![1,2,3];
let vslice = &vecu[..];
println!("{:?}",vslice);
}