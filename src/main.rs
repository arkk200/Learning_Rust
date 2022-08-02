fn match_coulours(rgb:(u32, u32, u32)){
    match rgb {
        // 특정 값만 볼 경우 안 볼 값들은 _ 처리 해준다.
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),

        // rgb 다 10 이상일 경우
        _ => println!("Every colour has at least 10")
    }
}

fn match_number(input: i32) {
    match input {

        // _와 마찬가지로 조건이 없으므로 이 코드밖에 실행이 안됨
        // number => println!("It's the number {}", number),

        // 슬라이스 때 썼던 걸 쓸 수 있음
        // 이 때 만약 값을 가져오고 싶으면 앞에 '변수명 @' 를 쓰면 된다.
        number @ 0..=10 => println!("It's between 0 and 10. It's the number {} ", number),
        _ => println!("It's greater than ten")
    }
}

fn main() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_coulours(first); // first같은 경우 match의 특성에 의해서 Not much gree만 출력된다.
    match_coulours(second);
    match_coulours(third);

    /*
    let 변수 = match할 때 match에서 반환하는 값들의 타입은 다 같아야 한다.
    */
    // let i = match 1 { 3 => 1, 1 => "1", _ => '샍' } // 이렇게 하면 에러가 발생함
    
    // 다른 예
    // 얘도 주는 타입이 다르므로 에러가 발생함
    // let j = if 10 == 9 { 8 } else { "Something else" }

    println!("");

    match_number(10);
    match_number(100);
}