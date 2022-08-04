// Option
// Result
// Ocaml이라는 언어에서 Option과 Result의 개념을 갖고 옴

// 다른 언어에선 포인터가 null을 가르키면 심각한 문제를 일으키는데
// 러스트에선 포인터가 무엇을 가르키는지 모를 땐
// Option을 선택하게 하면 된다.
// 가르키는게 없으면 None, 있으면 generic T를 가지고
// Some을 실행시킨다.
enum _Option<T>{
    None,
    Some(T),
}

// Option이 쓰이는 예제
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        // Option<i32>와 i32는 타입이 다르므로
        // Some으로 감싸줘야함
        Some(value[4])
    }
}
// 위와 같은 경우, Option을 쓰면 안전해진다.

fn main() {
    let new_vec = vec![1, 2, 0, 2, 2, 1, 0];
    let index = take_fifth(new_vec); // Option<i32>: Some(2)
    let another_new_vec = vec![1, 0];
    let another_index = take_fifth(another_new_vec); // None
    println!("{:?}", index);

    // .unwrap() - take out what is inside

    // Some()을 unwrap하면 안에 있는 값이 나옴
    // 그러나 None을 unwrap하면 panick이 뜸
    println!("{}", index.unwrap()); // 2
    // println!("{}", another_index.unwrap()); // panick 에러
    
    // 위의 panick 에러를 예방하기 위해 match를 씀
    match another_index {

        // Some() 이라면
        Some(number) => println!("I got a number: {}", number),

        // None 이라면
        None => println!("There was nothing inside")
    }

    // 다른 예방 방법
    if index.is_some() {
        // some이라면 실행되므로 unrwap해도 에러가 안남 
        println!("I got a number: {}", index.unwrap());
    }

    // 에러가 생겼을 때 expect를 이용하여 error 메세지를 만들 수 있다.
    another_index.expect("Needed at least five items");
}