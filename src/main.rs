// Question mark operator

use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    // ?는 match를 해주는데 에러가 있으면 error을 반환한다.
    // 물음표 연산자를 쓸 때 무조건 반환하는게 Result여야한다.
    // 에러가 있을 경우 에러를 반환하는데 그에 맞는 타입을 써줘야 하기 때문이다.
    let parsed_number = input.parse::<i32>()?; // return error
    println!("It worked! {}", parsed_number);

    // ? 는 에러가 생길 수 있는 위치에 더 써줄 수도 있다.
    let parsed_number = input.parse::<u16>()?.to_string().parse::<u32>()?.to_string().parse::<i32>()?;

    Ok(parsed_number) // Result를 반환하므로 이런식으로 써야한다.
}

fn main() { // 9.0은 안됨
    for i in vec!["Seven", "8", "9.0", "nice", "6060"] {
        let parsed = parse_str(i);
        println!("{:?}", parsed);
        // if let Ok(num) = parsed {
        //     println!("{}", num); // Result를 반환받으므로 Debug print를 씀
        // }
    }
}