use std::fmt::Display;
use std::cmp::PartialOrd;

/* 
fn _compare_and_print<T: Display>(statement: T, num_1: T, num_2: T) {
    println!(
        "{}! is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2 // 숫자 외의 타입은 비교가 안되니 에러가 뜬다.
    );
}
*/

fn _fixed_compare_and_print<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!(
        "{}! is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2 // 숫자 외의 타입은 비교가 안되니 에러가 뜬다.
    );
}

fn _simple_fixed_compare_and_print<T, U>(statement: T, num_1: U, num_2: U)
where // trait이 너무 길어지면 where을 써서 보기 쉽게 만들 수 있다.
    T: Display,
    U: Display + PartialOrd
{
    println!(
        "{}! is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2 // 숫자 외의 타입은 비교가 안되니 에러가 뜬다.
    );
}

fn main() {
    _simple_fixed_compare_and_print("Listen up", 19, 29);
}