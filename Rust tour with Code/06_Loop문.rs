fn main(){
    let mut i = 0;
    loop {
        i = i+1;
        println!("->{}<-",i);
        if i==10 {
            break;
        }
    }
}