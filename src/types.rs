/*
    Primative Types;
    Integers:u8,i8,u16,i16,u32,i32,u64,i64,u128,i128(numbers of bits they take in memeory)
    Floats:f32,f64(floats in memeory)
    boolean:(bool)
    Characters:(char)
    Tuples
    Arrays
 */
//Note: Rust is a statically types langauage,which means that it must know the types of all variables at compile time,however, the compiler can usally inferr what type we wnat to use based on the valye and how we use it
pub fn run(){
    //Default x will be "i32"
    let x = 1;
    //Default y will be "f32"
    let y = 2.5;

    //Add explicit type
    let z:i64=455555555;

    // find max sixe 
    println!("Max i32: {}",std::i32::MAX);
    println!("Max i64: {}",std::i64::MAX);
    println!("Max f32: {}",std::f32::MAX);

    //Boolen
    let is_active = true;

    //Getting boolean from expression
    let is_greater:bool = (x as f32)>y;
    
    // Characters
    let a = 'a'; //char is use single qoute to define char
    let face = '\u{1F600}'; 

    println!("Is active: {:?}",(x,y,z,is_active, is_greater,a,face));

}