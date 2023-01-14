pub fn run(){
    let bird = Bird{
        name: String::from("Parrot"),
        age: 2,
        color: String::from("Green"),
    };
    bird.print_name();
    println!("{} {}", bird.can_fly(),bird.is_animal());
    }

struct Bird{
    name: String,
    age: u8,
    color: String,
}

impl Bird {
    fn print_name(&self){
        println!("Bird name is {}", self.name);
    }
}

impl Animal for Bird {
    fn can_fly(&self)->bool {
        true
    }
}
trait Animal {
    fn can_fly(&self)->bool;
    fn is_animal(&self)->bool{
        true
    }
}