fn number() -> i32 {
    10 // ; // 세미콜론을 쓰면 () (empty tuple)이 나온다.
}

// () - 빈 튜플 (empty tuple), unit type (void)

fn empty_tuple1() -> (){} // 직접적으로 튜플을 반환하거나
fn empty_tuple2(){
    // return 값이 없거나
}
fn empty_tuple3(){
    1; // 세미콜론을 쓰면 empty tuple이 나온다.
}

// Display {}
// Debug {:?}
fn main() { // main() 함수도 return 값이 없으니 empty tuple을 반환한다고 할 수 있다.
    let _x = number();
    println!("{}", _x);
    let _tuple1 = empty_tuple1();
    let _tuple2 = empty_tuple2();
    let _tuple3 = empty_tuple3();
    println!("{:?}", _tuple1); // 튜플은 디버그로 출력해야한다.
}