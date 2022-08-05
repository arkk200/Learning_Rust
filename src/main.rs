struct Animal {
    name: String
}

// trait도 struct처럼 만들면 된다.
trait Canine { // dog-like
    // 형식이 똑같으면 impl에서 덮어쓸 수 있음
    fn bark(&self) {
        println!("I was changed by impl fn");
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
        println!("Woof woof!");
    }
}

fn main() {
    let my_animal = Animal {
        name: "Mr. Mantle".to_string()
    };

    my_animal.bark();
    my_animal.run();
}