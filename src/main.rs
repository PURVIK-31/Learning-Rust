//iterators in rust

// defining a iterator
// iterator is a type of rust that implements the iterator trait
// iterator trait is defined in the standard library
// iterator trait has a method called next that returns an option
// option is an enum that has two variants Some and None
// Some contains the next value and None indicates that the iterator is finished    

//iterators are lazy, they do nothing until you call the next method
//iterators are also composable, you can chain them together to perform complex operations

fn main(){
    let val = vec![1,2,3,4,5];
let vak = val.iter();       
    for i in vak{
    println!("{}",i);
}

}