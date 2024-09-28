// enum Animal {
//     Dog,
//     Cat,
//     Duck
// }


// fn make_sound(animal: Animal) {
//     match animal {
//         Animal::Dog => println!("Woof"),
//         Animal::Cat => println!("Meow"),
//         Animal::Duck => println!("Quack")
//     }
// }

trait Animal {
    fn vocalize(&self) -> ();
}

struct Dog {}

impl Animal for Dog {
    fn vocalize(&self) {
        println!("Woof");
    }
}

struct Cat {}

impl Animal for Cat {
    fn vocalize(&self) {
        println!("Meow");
    }
}

struct Duck {}

impl Animal for Duck {
    fn vocalize(&self) {
        println!("Quack");
    }
}

fn make_sound(animal: impl Animal)  {
    animal.vocalize()
}
