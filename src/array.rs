//Arrys-fixed lsit where elements are the same data types
use std::mem;
pub fn run(){
    let numbers:[i32;5]=[1,2,3,4,5];
    println!("{:?}",numbers);
    //Get Single Value
    println!("Single value: {}",numbers[2]);
    //Get Array Length
    println!("Array Length: {}",numbers.len());
    
    //Array are stack allocated
    println!("Array occupies {} bytes" ,mem::size_of_val(&numbers));
    
    //Get slice 
    let slice:&[i32]=&numbers[0..2];
    println!("Slice {:?}",slice);
    
}