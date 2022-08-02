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

impl Animal {
    // new는 러스트에서 키워드가 아님 써도 상관없음
    // Self == Animal, Self는 struct Animal을 가르킴
    fn new() -> Self { // function signature
        Self {
            age: 10,
            animal_type: AnimalType::Cat
        }
    }
    fn new_cat(age:u8) -> Self { // function signature
        Self {
            age,
            animal_type: AnimalType::Cat
        }
    }
}

fn main() {
    // new()를 호출하면 impl에 있는 fn new의 Self가 반환됨
    // associated function, 타입하고 관련된 함수이며 아직 메서드가 안됨
    let my_animal = Animal::new();
    println!("{:?}", my_animal);
    let my_animal = Animal::new_cat(5);
    println!("{:?}", my_animal);
}