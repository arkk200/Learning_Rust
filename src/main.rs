// 함수에서 반환하고자 하는 타입은 '함수명() -> 타입 {}' 형식으로 쓰고
// return할 표현식은 return을 써도 되지만 암묵적으로 마지막 표현식을 반환한다.
fn give_age() -> i32 {
    42
}
// !(macro) = 코드를 쓰는 함수
fn main() {
    println!("ASDF");
    // 매크로는 아래처럼 써야할 복잡한 코드를 자동을 써주는 함수이다.
    // { ::std::io::_print(::core::fmt::Arguments::new_v1(&["ASDF\n"], &[])); };
    let my_name = "LMJ";
    // let my_age = 42;
    println!("My name is {} and my age is {}", my_name, give_age());
}