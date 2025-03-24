//lets start with some advanced rust concepts :

//collections framework in rust!


//vectors --> dynamic arrays : 

fn main(){
 let mut vec = Vec::new();
 vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
 printeven(vec);
}

fn printeven(vec:Vec<i32>){
vec.iter().for_each(|x: &i32| {
    if x%2==0{
        println!("{}",x);
    }
});
}