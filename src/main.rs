// Option은 generic이 존재하면 Some, 그렇지 않다면 None을 나타낸다.

// Result

// enum Result<T, E> {
//     Ok(T), // 잘 된다면 T
//     Err(E) // 안되면 E를 이용해서 에러메세지를 출력하거나 등등을 할 수 있다.
// }

use std::{slice::ChunksExact, path::Ancestors};

// 형식상 이렇게 써도 상관없다.
fn _check_error() -> Result<(), ()> {
     Ok(())
}

// 얘도 형식상 Result를 잘 쓰는거다.
fn _another_check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 { // 에러를 정할 수 있다.
        Ok(())
    } else {
        Err(())
    }
}

// Option에는 .is_some(), .is_none() 메서드가 있듯이
// Result에도 .is_ok(), .is_err() 메서드가 있다.

fn main() {
    let _var = _check_error();
    let _var = _another_check_error(5);
    if _var.is_ok() {
        println!("It's okay, guys!");
    } else {
        println!("It's an error, guys!");
    }
    match _another_check_error(10) {
        // Ok(), Err() 안에 유닛 타입이 있는데
        // 필요 없으니 _로 처리한다.
        Ok(_) => println!("Okay guys"),
        Err(_) => println!("It's an error")
    }
    // Option과 똑같이 .unwrap()을 쓸 수 있다.
    // Err가 나온다면 panic이 뜬다.
    _another_check_error(10).unwrap();

    // Option: None.unwrap() -> panic
    // Result: Err.unwrap() -> panic
}