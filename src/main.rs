/*

*/

fn main() { // &: 참조를 만듦
    let my_variable = 8;
    // single_reference가 my_variable의 메모리 주소를 참조함
    let single_reference = &my_variable; // &i32
    // double_reference가 single_reference의 메모리 주소를 참조함
    let double_reference = &single_reference; // &&i32
    let five_reference = &&&&&my_variable; // &&&&&i32
    /*
    double_reference가 single_reference의 메모리 주소를 알고 있고
    single_reference가 my_variable의 메모리 주소를 알고 있음
    // 하지만 타입은 서로 다르다
    */
    println!("{}, {}", double_reference, five_reference);
    // if double_reference == single_reference {} // 타입이 다르기에 서로 비교할 수 없음
}