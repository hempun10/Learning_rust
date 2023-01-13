//Tuples group together values of differnt types
//Max 12 element
pub fn run(){
    let person:(&str,&str,i8)= ("Heemal","Magar",18);
    println!("{} {} is {} years old ",person.0,person.1,person.2)
}