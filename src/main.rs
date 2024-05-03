trait Animal {
    fn make_sound(&self);
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Bark!");
    }
}

struct Dog;

trait Cat: Animal {
    fn purr(&self);
}

struct Moesha;

impl Animal for Moesha {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

impl Cat for Moesha {
    fn purr(&self) {
        println!("Purr!");
    }
}

fn describe(animal: &dyn Animal) {
    animal.make_sound();
}

fn main() {
    let dog = Dog;
    let moesha = Moesha;

    describe(&dog);

    moesha.make_sound();
    moesha.purr();
}