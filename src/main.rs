// 매개변수는 '매개변수: 타입' 형식으로 타입을 정해주어야 한다.
fn give_number(one: i16, two: i16) -> i16 {
    let multipleied_by_ten = { // 블럭만 만들어도 함수같이 만들 수 있다.
        let first_number = 10;
        first_number * one * two
    };
    multipleied_by_ten
}

fn main() {
    let my_number = give_number(7, 8);
    println!("{}", my_number);
}