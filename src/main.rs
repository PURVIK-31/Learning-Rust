//into intertor is a special type of iterator which has few benefits with conditions as per belowl:

// this trait converts the collection to an iterator and takes the ownership of the collection

//useful when:
//one no longer needs the original collection
//avoiding references and take performance benefits 
//avoiding lifetime issues


fn main() {
    let v = vec![1, 2, 3];
    let sum: i32 = v.into_iter().sum();
    println!("sum: {}", sum);
}