fn main() /*-> ()*/ { // main함수에선 안보이는 튜플을 반환하고 있다.
    let my_tuple = (8, "lmj", vec![1, 2, 3]); // tuple은 소괄호만 써서 만들 수 있다.
    println!("{:?}", my_tuple); // tuple은 다 똑같은 타입일 필요가 없다.
    // tuple내에 요소의 타입을 알고 싶으로 뒤에 아무글자의 메소드를 호출하면 된다.
    // my_tuple.wefuhwaeogv();
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: First item: {:?}
Seconde item: {}
Third item: {:?}
Fourth item: {:?}
Fifth item: {:?}
Sixth item: {}", // 정수, 실수는 디버그 프린트를 쓸 필요가 없다.
        random_tuple.0, // 튜플이 Vec, Array와 다른 점은 요소를 가져올 때 프로퍼티를 이용해서 가져온다.
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    );

    // Vec에 두 개의 자료형을 넣게하고 싶으면 tuple을 쓰면 된다
    // Vec<(String, i32)>
    let _my_vec = vec![("Hey", 9), ("Hello there", 321241)];

    // Destructuring
    let str_tuple = ("one", "two", "three");
    let (a, b, c) = str_tuple; // 러스트에도 비구조화 할당을 할 수 있다.
    let (d, _, _) = str_tuple; // _는 할당을 안 받을 경우 쓴다. 또한 요소 개수만큼 변수를 개수를 맞춰 줘야한다.
    println!("{}, {}, {}, {}", a, b, c, d);
}