fn main(){
    let vec1 = vec![1, 2, 3];
    let iter  = vec1.iter();
    let filtered_iter = iter.filter(|x| *x % 2 == 0);
    print!("{:?}", filtered_iter.collect::<Vec<&i32>>());  
}