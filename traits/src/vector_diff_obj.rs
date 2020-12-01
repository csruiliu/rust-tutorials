#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

trait Animal {
    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("Hello, my name is {}", self.name());
    }
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

/**
enum Creature {
    Human(Human),
    Cat(Cat)
}
*/

fn main() {
    
    /**
     * first way to implement vector with different objectives
     * need enum data structure
     * 
    let mut creatures:Vec<Creature> = Vec::new();
    creatures.push(Creature::Human(
        Human{name:"John"}
    ));
    creatures.push(Creature::Cat(
        Cat{name:"Fluffy"}
    ));

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk()
        }
    }
    */
    
    // second way to implement vector with different objectives
    let mut animals:Vec<Box<Animal>> = Vec::new();
    animals.push(Box::new(Human{name:"John"}));
    animals.push(Box::new(Cat{name:"Fluffy"}));

    for a in animals.iter() {
        a.talk();
    }
}