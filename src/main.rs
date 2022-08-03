// impl blocks (implement blocks)

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType
}

#[derive(Debug)]
enum AnimalType {
    Cat(String),
    Dog(String)
}

impl AnimalType {
    fn print_name(&self) { // enum에도 impl을 만들 수 있다.
        match self {
            AnimalType::Cat(name) => println!("Animal type is cat and name is {}", name),
            AnimalType::Dog(name) => println!("Animal type is dog and name is {}", name)
        }
    }
}

impl Animal { // 같은 이름의 impl을 더 만들어도 된다.
    // fn new_old_cat() -> Self {
    //     Self { age: 15, animal_type: AnimalType::Cat }
    // }
}

impl Animal {
    fn new(age:u8, animal_type: AnimalType) -> Self {
        Self { age, animal_type }
    }

    // fn new_cat(age:u8) -> Self { // function signature
    //     Self {
    //         age,
    //         animal_type: AnimalType::Cat
    //     }
    // }

    // fn new_dog(age:u8) -> Self { // function signature
    //     Self {
    //         age,
    //         animal_type: AnimalType::Dog
    //     }
    // }

    // fn print(&self) {
    //     println!("I am a: {:?}", self);
    // }

    // fn change_to_dog(&mut self){
    //     self.animal_type = AnimalType::Dog;
    //     println!("Changed to dog! Now I am: {:?}", self);
    // }

    // fn change_to_cat(&mut self){
    //     self.animal_type = AnimalType::Cat;
    //     println!("Changed to cat! Now I am: {:?}", self);
    // }
}

fn main() {
    use AnimalType::*;
    let my_cat = Animal::new(10, Cat("naaaame".to_string()));
    my_cat.animal_type.print_name();
}