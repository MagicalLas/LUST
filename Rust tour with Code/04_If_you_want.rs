fn main(){
    let n = 34;

    if(n>10){
        println!("->{}<-",n);
    }else{
        
    }

    let c = {
        if(n>20){
            n*20
        }
        else{
            n+2
        }

    };
    println!("->{}<-",c);
}

