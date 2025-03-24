//iterators in rust

// defining a iterator
// iterator is a type of rust that implements the iterator trait
// iterator trait is defined in the standard library
// iterator trait has a method called next that returns an option
// option is an enum that has two variants Some and None
// Some contains the next value and None indicates that the iterator is finished    

use std::any::type_name_of_val;


fn main(){
    let val = vec![1,2,3,4,5];
    let iterator  = val.iter();
    let typeofiter = type_name_of_val(&iterator);
    println!("{}",typeofiter);

}