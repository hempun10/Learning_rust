pub fn run(){
    let bird = Bird{
        name: String::from("Parrot"),
        age: 2,
        color: String::from("Green"),
    };
    bird.print_name();
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