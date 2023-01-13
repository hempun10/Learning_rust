//Vectors-Resizable Array
use std::mem;
pub fn run(){
    let mut numbers:Vec<i32>=vec![1,2,3,4,5];
      //Add on to vector
      numbers.push(6);
      numbers.push(7);
    //   Re-assign value 
    numbers[2]=20;
    //Pop off the last value
    numbers.pop();
    //print vector
    println!("{:?}",numbers);
    //Get Single Value
    println!("Single value: {}",numbers[2]);
    //Get Array Length
    println!("Vector Length: {}",numbers.len());
    //Array are stack allocated
    println!("Vector occupies {} bytes" ,mem::size_of_val(&numbers));
    //Get slice 
    let slice:&[i32]=&numbers[0..2];
    println!("Slice {:?}",slice);
    // loop through vector value 
    for x in numbers.iter(){
        println!("Number: {}",x);
    }
    //loop and muted values
    for x in numbers.iter_mut(){
        *x*=2;
    }
    println!("Numbers Vec: {:?}",numbers);

}