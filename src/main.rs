fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0{
            break;
        }
    }
    counter
}

// uninitialized variable 초기화 되지 않은 변수
// control flow

fn main() {
    let _number = 8; // 변수는 마지막 코드일지라도 세미콜콘을 써주어야 한다.
    let _my_second_num: u16; // 초기화 되지 않은 변수, 값이 없어서 쓸 수가 없음
    // 블럭 안에서 값을 수정해도 변수가 선언된 위치를 변수의 활동 범위로 잡음
    { _my_second_num = 10; } // 전에 말했듯이 최초 1회까지 값을 할당해주기만 하면 된다.
    println!("{}", _my_second_num);

    let _my_variable = {
        let x = loop_then_return(43);
        x
    };
    println!("{}", _my_variable);
}