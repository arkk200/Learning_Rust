use std::fmt;

struct Animal {
    name: String
}

// trait도 struct처럼 만들면 된다.
trait Canine { // dog-like
    // 형식이 똑같으면 impl에서 덮어쓸 수 있음
    fn bark(&self) {
        println!("I was changed by impl fn");
        // Animal에 있는 name을 가져올 것 같지만
        // 다른 struct도 이 trait을 쓸 수 있기에 오류가 남
    }
    fn run(&self) { // 여기서 바로 default implementation도 할 수 있다.
        println!("I am running!");
    }
}

// impl에서 trait에 있는 함수를 struct에게 줄 땐
// 'impl trait명 for 구조체명' 형식으로 쓰면 된다.
impl Canine for Animal {
    // trait Canine에서 적었던 함수를 imple에서 더 자세히 적음
    fn bark(&self) {
        // 여기선 명시가 되어 있으므로 (impl Canine for Animal)쓸 수 있다.
        println!("Woof woof! 나는 {}라고 한다", self.name);
    }
}

// --------------------

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8
}

impl fmt::Display for Cat { // trait를 써서 Display print도 할 수 있다.
    // 함수 이름은 꼭 fmt로 써야함
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let age = self.age;
        write!(f, "My cat's name is {name} and it is {age} years old")
    }
}

fn main() {
    let my_animal = Animal {
        name: "Mr. Mantle".to_string()
    };

    my_animal.bark();
    my_animal.run();

    // --------------------

    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4
    };

    println!("{mr_mantle}");
}