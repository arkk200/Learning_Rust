fn main() {
    let mut number = 10;
    /*
    let number_ref = &number;
    let number_change = &mut number_ref;
    *number_change += 10; // immutable 참조와 mutable 참조를 같이 쓰면 에러가 남
    */
    // 비슷한 상황인데 에러가 안나는 경우
    let number_change = &mut number;
    *number_change += 10;
    let number_ref = &number;
    let _number_ref2 = &number;
    let _number_ref3 = &number;
    // immutable 참조와 mutable 참조를 같이 쓸 때
    // immutable 참조를 많이 해도 mutable 참조가 하나라면
    // 에러가 나지 않는다.
    println!("{}", number_ref);
    
    // shadowing
    let country = "my country";
    let country_ref = &country;
    let country = 8;
    // 참조하는 변수는 변수를 참조하는 것이 아니고 값을 참조하는 것이다.
    println!("{}, {}", country_ref, country); // my country, 8
}