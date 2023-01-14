use std::collections::HashMap;

//None of the indicate failure or lack of value
//Some(value) , a tuple struct that warps a value with type T.

fn divide(dividend:i32,divisor:i32)-> Option<i32>{
    if dividend % divisor !=0{
        None
    } else{
        Some(dividend/divisor)
    }
}
pub fn run(){
    let divide1 : Option<i32> = divide(4, 2);
    let divide2 : Option<i32> = divide(4, 3);

    //Unwrapping a `Some` varient will extract the value wrapped
    println!("Divide1: {:?} unwraps to {}", divide1,divide1.unwrap());

    //Unwrapping a `None` varient will cause the program to crash or `panic`
    println!("Divide2: {:?} unwrap to {}", divide2,divide2.unwrap());
}