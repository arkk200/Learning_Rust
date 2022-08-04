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
    let new_vec = vec![1, 2, 4, 7, 8, 10, 10];
    let index = take_fifth(new_vec);
    println!("{:?}", index);
}