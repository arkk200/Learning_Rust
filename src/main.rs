use std::num::ParseIntError;

// 5가 나오면 Ok, 아니면 Err가 나오는& 함수
fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Error!! It's not five".to_string())
    }
}

// Result<T, E>타입을 반환할 땐 Ok(T), Err(E)를 반환해야한다.
fn parse_number(num: &str) -> Result<i32, ParseIntError> {
    // 숫자만 있는 문자열이 들어가야 에러가 안 남
    num.parse() // parse는 자기만의 에러를 가지고 있음: ParseIntError
}

fn main() {
    // Vec<Result<i32, String>>
    let mut result_vec = Vec::new();

    for number in 2..=7 {
        result_vec.push(check_if_five(number));
    }

    println!("{:?}", result_vec);

    // parse - 숫자로 만들어줌

    let mut my_vec = vec![];
    my_vec.push(parse_number("8"));
    my_vec.push(parse_number("aweofi")); // 에러가 남
    my_vec.push(parse_number("10"));

    for num in my_vec{
        println!("{:?}", num);
    }
}