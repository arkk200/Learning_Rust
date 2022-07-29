fn main() {
    /*
    부호가 있는 정수: i8, i16, i32, i64, i128, isize
    부호가 없는 정수: u8, u16, u32, u64, u128, usize
    i, u 뒤에 숫자는 2의 n비트를 나타낸다. ex) i8 = 2의 8승 비트의 부호가 있는 정수
    isize, usize는 computer architecture에 따라서 정해진다
    32비트 컴퓨터는 isize, usize가 32비트(i32, u32)가 되고,
    64비트 컴퓨터는 isize, usize가 64비트(i64, u64)가 된다.
    */
    let _my_number1: u8 = 100; // 타입은 let/const 변수명: 타입 = value; 형식으로 적는다.
    let _my_number2 = 100; // 어떤 타입인지 컴파일러한테 알려주지 않으면 자동으로 i32가 된다.
    let _another_number1: u16 = 10;

    /*
    let _third_another_number = _my_number1 + _another_number1; // 부호가 같아도 타입이 다르면 안된다.
    */
    /* 
    let _third_number = _my_number1 + _my_number2; // 러스트에선 다른 타입끼리 연산할 수 없다.
    */
    // type inference
}