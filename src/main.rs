use core::num;

enum Star {
    // 컴파일러가 보는 숫자대신 다른 숫자를 넣어 바꿀 수 있다.
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar // 전 값에 값을 대입해주었다면 값을 대입 안 한 이 값은 전 값에 +1 한 값이 된다.
}

// --------------------

enum Number {
    // enum은 데이터 타입을 가짐, 안에 속성들도 대문자부터 시작해야함
    U32(u32),
    I32(i32)
}

fn get_number(input: i32) -> Number {
    match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input)
    }
}

fn main() {
    use Star::*;

    // Vec<Star>
    let stars = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];

    for star in stars {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star"),
            size if size > 80 => println!("Pretty big star: {}", size),

            // 원래 위의 코드만 써줘도 문제가 없는데 아직 구현이 안된 것으로 보임
            // 따라서 _를 써줘서 에러를 없애줌
            _ => println!("Some other star")
        }
    }
    
    println!("What about DeadStar? Is is: {}", DeadStar as u32); // 1001

    // --------------------
    println!("");

    // Vec<Number>
    // get_number(값)을 호출하고
    // 양수면 Number::U32, 그렇지 않다면 Number:I32를 반환받는다.
    // 받은 값을 토대로 Vec을 만든다.
    let my_vec = vec![get_number(-800), get_number(8), get_number(-600)];

    for item in my_vec { // 만든 Vec를 for문을 이용해서 하나씩 돌림
        match item { // Number::U32(u32)이면 첫번째 코드를, Number::I32(i32)이면 두번째 코드를 실행한다.
            Number::U32(number) => println!("It's a u32 with the value: {}", number),
            Number::I32(number) => println!("It's a i32 with the value: {}", number)
        };
    }
}