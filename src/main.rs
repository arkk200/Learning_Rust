// loops

fn main() {
    let mut counter = 0;

    // loop는 러스트와 맞지 않기에 while을 쓰는게 낫다
    while counter != 5 { // '루프명: loop 로 이름을 지정해줄 수 있다.
        counter += 1;
        println!("The counter is: {}", counter);
    }

    // ---------------

    // Range: 러스트에서 range는 n..m 식으로 쓴다.
    // exclusive Range: n..m
    // inclusive Range: n..=m
    // 값을 받는 변수도 앞에 _를 붙여서 컴파일러가 무시할 수 있게 할 수 있다.
    for number in 0..3 { // exclusive Range
        println!("The number is {}", number);
    }

    // ---------------

    let mut counter = 5;

    let my_number = loop {
        counter += 1;
        if counter % 53 == 3{
            break counter; // break하면서 counter를 return함
        }
    };
    println!("my_number is now: {}", my_number);
}