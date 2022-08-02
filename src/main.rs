// Vec는 Vector를 줄인말이다.
// Array는 [u8: 10]처럼 배열의 크기에 따라 각각 다른 타입이지만
// Vec은 Vec<String>처럼 크기가 없다.
fn main() {
    // 015번 커밋에서 String.capacity()는 벡터의 크기를 출력하는 함수였다.
    let _my_string = String::new();
    // Vec도 String만들듯이 Vec::new()로 만들 수 있다.
    let _my_vec: Vec<String> = Vec::new(); // 또한 Vec<타입> 형태로 자료형을 부여한다.
    // let _my_error_vec = Vec::new(); // 자료형을 쓰지 않으면 에러가 발생한다.
    // 에러에 type annotations needed for `Vec<T>`이라는 내용이 나오는데 Vec<T>에서 T는
    // 어떤 타입을 의미한다. 꼭, T가 아닐 수도 있다.
    let name1 = String::from("WINDY");
    let name2 = String::from("GOMESY");

    let mut my_vec = Vec::new();
    println!("Space for my_vec: {}", my_vec.capacity()); // 0
    my_vec.push(&name1); // 타입을 가진 요소를 자료형이 없는 Vec에 push하면 에러가 나지 않는다.
    println!("Space for my_vec: {}", my_vec.capacity()); //
    my_vec.push(&name2);
    println!("Space for my_vec: {}", my_vec.capacity()); //

    println!("My cats are {:?}", my_vec);

    let my_vec = vec![&name1, &name2]; // Vec::new, push 없이 선언을 바로 할 수 있다.
    println!("My cats are {:?}", my_vec);
}