// impl blocks (implement blocks)

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog
}

impl Animal { // 같은 이름의 impl을 더 만들어도 된다.
    fn new_old_cat() -> Self {
        Self { age: 15, animal_type: AnimalType::Cat }
    }
}

impl Animal {
    fn new_cat(age:u8) -> Self { // function signature
        Self {
            age,
            animal_type: AnimalType::Cat
        }
    }
    fn new_dog(age:u8) -> Self { // function signature
        Self {
            age,
            animal_type: AnimalType::Dog
        }
    }

    fn print(&self) {
        println!("I am a: {:?}", self);
    }

    fn change_to_dog(&mut self){
        self.animal_type = AnimalType::Dog;
        println!("Changed to dog! Now I am: {:?}", self);
    }
    fn change_to_cat(&mut self){
        self.animal_type = AnimalType::Cat;
        println!("Changed to cat! Now I am: {:?}", self);
    }
}

fn main() {
    // new()를 호출하면 impl에 있는 fn new의 Self가 반환됨
    // associated function, 타입하고 관련된 함수이며 아직 메서드가 안됨
    let mut my_animal = Animal::new_dog(5);
    my_animal.print(); // syntactic sugar
    my_animal.change_to_cat();
    my_animal.change_to_dog();
    // 아래 방법을 쓰기 귀찮으니깐 위와 같은 syntactic sugar 방식을 쓴다.
    Animal::print(&my_animal);

    let my_old_cat = Animal::new_old_cat();
    println!("{:?}", my_old_cat);
}