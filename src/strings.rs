//Primitative Str = Immutable fixed-length string somewhere in memeory
//String = Growable, heap-allocated data-structure -Use whrn you need to modify or own
pub fn run(){
    // let name = "Heemal"; :> It is primative way to declaring string
    let mut name = String::from("Heemal "); 

    //Get length
    println!("Length: {}", name.len());

    //Push char 
    name.push('D');

    //Pushing  string
    name.push_str("on");
    // Note: we can push string and char only is the string is Growable String '&str'

    //Capacity in bytes
    println!("Capacity:{}",name.capacity());

    //Empty or not
    println!("Empty: {}", name.is_empty());

    //Conatins
    println!("Conatins:{}",name.contains("Don"));

    //Replace
    println!("Replace:{}",name.replace("Don","is Gem"));
    
    //Loop through string by whitespace
    for hi in name.split_whitespace(){
        println!("{}",hi);
    }

    //Create String with capacity
    let mut last_name = String::with_capacity(10);
    last_name.push_str("Maga");
    last_name.push ('r');

    //Assertion testing
    // assert_eq!(name,last_name);
    assert_eq!(10,last_name.capacity()); //for checking whether the  values of one var is equal to another var 
    //Print defalult 
    println!("{}", name);
    println!("{}", last_name);

}