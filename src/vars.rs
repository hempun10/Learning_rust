//Variables hold primitative data or refrences to data
// variables are immutable by default 
// Rust is a block sccopeed language 
pub fn run(){
    let name ="Heemal";
    let mut age =18; //Variable cannot be reassigned so in rust vars are immitable but if we want to reassigned variable we should used 'mut' keyword on declaring var
    println!("My name is {}. I am {} years old.", name,age);
    age = 19;
    let height =5.7;

    //Define const
    const ID:i32 =001; //its is compulsary to define datatype while defining const

    //Assign multiple vars
    let(fav_car,dream)=("Tesla Model Y","Insprition for other");
    println!("My name is {}. I am {} years old. My height is around {}ft . My id is {}. My favouriate car is {} and my dream is to be a {}", name,age,height,ID,fav_car,dream);
}