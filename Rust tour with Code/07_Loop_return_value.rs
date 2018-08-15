fn main(){
    let mut i = 0;
    let result = loop{
        i+=1;
        if i == 5{
            break i;
        }
    };
    println!("Result is {}", result);
}