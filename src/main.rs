// destructuring, 비구조화
// destructuring은 튜플, 배열로도 할 수 있다.

struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool
}

#[derive(Debug)]
struct Person2 {
    name: String,
    height: u8
}

impl Person2 { // 이런 식으로 destructuring을 활용하여 다른 정보만 가지는 구조체를 만들 수 있음
    fn from_person(input: Person) -> Self {
        // 뒤에 ..을 쓰면 나머지는 필요가 없다는 뜻이 됨
        let Person {name, height, ..} = input;

        Self { name, height }

    }
}

fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false
    };
    /*
    println!("They call him {}, but his real name is {}. He is {} cm tall and is he happy? {}",
        papa_doc.name,
        papa_doc.real_name,
        papa_doc.height,
        papa_doc.happiness
    );
    */

    // 'let 구조체이름 {키명1, 키명2, 키명3...} = 구조체 변수명' 으로 비구조화 할 수 있다.
    // {키명1: 다른이름1, 키명2: 다른이름2, 키명3: 다른이름3} 으로 변수 이름을 설정할 수 있다.
    let Person { name: a, real_name: b, height: c, happiness: d } = &papa_doc;
    println!("They call him {}, but his real name is {}. He is {} cm tall and is he happy? {}", a, b, c, d);

    println!("-------------------------");

    let person2 = Person2::from_person(papa_doc);
    println!("Person2 type is: {:?}", person2);

}