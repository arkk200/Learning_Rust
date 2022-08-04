// generics <-> concrete

// concrete: i32, String...

// angle bracket(<>)이 있으면 러스트 컴파일러가 generic타입으로 인식함

// 꼭 T일 필요는 없음
fn give_thing<T>(input: T) -> T { // T
    input
}

fn _another_give_thing<T>(input: T) -> T {
    // generic이 받은 자료형이 만약 구조체라면
    // Display, Debug 출력도 안되기에 이 코드는 안전하지 않다
    // println!("{}", input);
    input
}

use std::fmt::Display;
// 만약 Display로 출력하고 싶다면 generic에 
// Display trait이 있어야 하기에 std::fmt::Display를 적어준다.
fn fixed_give_thing<T: Display>(input: T) -> T {
    println!("{}", input);
    input
}

struct _Book;
fn main() {
    // generic은 여러가지 자료형을 받을 수 있다.
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
    println!("{}", x);
    println!("{}", y);

    fixed_give_thing(123);
    
    // 그러나 fixed_give_thing은 Display 출력이 가능한 타입만 받기에
    // struct를 주면 에러가 난다.
    // fixed_give_thing(Book);
}