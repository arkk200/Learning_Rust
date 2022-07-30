// 러스트도 전역변수, 지역변수가 있다.
// let은 전역변수로 쓸 수 없는 모양이다.
// const
const _NUMBER: i32 = 20; // const는 컴파일러가 알아서 타입을 맞추지 않기에 타입을 무조건 써야 한다. 또한 const는 다 대문자로 써주어야 한다.
// static
// static도 마찬가지로 타입을 써줘야 하며, 전부 대문자로 써줘야 한다.
static mut _LOW_SCORE: i32 = 0;
// static보단 const를 많이 쓴다.
static mut _CAN_MUTABLE: i32 = 0; // const와 다르게 static은 mut를 같이 쓸 수 있다.
// 하지만 안전하지 않기에 안 쓰는 것을 권장한다. --(1), 웬만하면 rust에서 unsafe를 안쓰는게 좋다.

fn print_high_score() {
    println!("The high score is {}", _NUMBER);
}
fn main() {
    let _x = 8; // 'let' binding: i32, let은 컴파일러가 알아서 타입을 맞추기에 타입을 쓸 필요가 없다.
    print_high_score();
    let _y = unsafe {_CAN_MUTABLE}; // (1)-- 그렇기에 만약 쓰게 된다면 unsafe{} 안에 변수를 넣어주어야 한다.
}