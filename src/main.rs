// &: immutable reference / shared reference
// &mut는 값을 참조하는데 그 참조한 값을 바꿀 수가 있다.
/* 하나의 변수를 참조하는 변수가
여러 개일 경우 위험할 수 있기에
하나만 쓰는게 안전하다. */
// &mut: mutable reference / unique refernce

fn main() {
    // &mut로 참조당한 변수의 값을 바꿔야 하므로
    // 참조 당하는 변수도 mut를 써주어야 한다.
    let mut my_number = 9;
    // num_ref가 my_number의 값을 바꿀 수 있음
    let num_ref = &mut &mut my_number;
    **num_ref = 10;
    // my_number이 10으로 바뀜
    println!("Changed number by num_ref is {}", my_number);
    // mutable reference와 immutable reference는 같이 쓰면 안전하지 않기에
    // 같이 쓸 수 없다.
}