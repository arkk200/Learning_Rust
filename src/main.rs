fn main() {
    let _my_number = 9u16; // 숫자 뒤에 타입을 추가하여 정할& 수 있다.
    let _prettry_my_number = 9_u8;
    let _other_number = 1_000_000_000_000u64; // 언더바는 키워드 빼고는 다 무시한다.
     // 정수에서 default타입은 i32였다면실수에서 default타입은 f64이다. 8바이트
    let _my_float_number_with_dot = 9.; // f64, .만 찍어도 컴파일러가 실수로 판단한다
    let _my_float_number = 9.999; // f64, .만 찍어도 컴파일러가 실수로 판단한다
    let _my_int_number = 9; // i32
    println!("{}", _my_float_number as i32 + _my_int_number); // 실수를 정수로 변환하면 소수점이 사라진다.
    println!("{}", _my_int_number as f64); // 정수를 실수로 변환해도 아무 문제 없다.
}