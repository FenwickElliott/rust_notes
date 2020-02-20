#[derive(Debug)]

struct Dog {
    name: String,
}

impl Dog {
    fn bark() {
        println!("woof!");
    }

    fn call(&self) {
        println!("come {}!", self.name);
    }
}

fn main() {
    let twigg = Dog{name: String::from("pringle")};

    println!("{:?}", twigg);
    Dog::bark();
    twigg.call();
}