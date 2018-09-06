use std::mem;

fn main(){
    let arr: [i64;8] = [1,2,3,4,5,6,7,8];
    println!("->{}", arr[3]);
    println!("->{}", arr.len());
    println!("->{}", arr[2 .. 5].len());
    println!("->{}", mem::size_of_val(&arr[3]));
    println!("->{}", mem::size_of_val(&arr));
}