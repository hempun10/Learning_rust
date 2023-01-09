pub fn run(){
    // print to console 
    println!("Hello from the print.rs file ");
    
    //Basic Formating
    println!("{} is {}","Hem","Gem");
    
    //Positing Arguments
    println!("{0} is {1} and {0} likes to {2}","Hem","Gem","code"); //Positing args according to index

    //Named Arguments
    println!("{name} likes to play {activity}",name="Hem",activity="football");

    //Placeholder tratis
    println!("Binary:{:b} Hex:{:x} Octal:{:o}",20,50,90); //Will give the binary,hex,and oct value of the args respectively.

    //Placeholder for debug tratis
    println!("{:?}",(1,3,4,true,false,"Hello world","Himal!")); //Its is used to print the array. This is also know as tuple

    //Basic Math
    println!{"10 + 10 = {}",10+10};

}