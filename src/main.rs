fn main() {
    // C언어와 똑같이 char는 작은따옴표를 쓰고
    // 문자열은 큰따옴표를 쓴다.
    println!("Hello, world!");
    let _char = 'A'; // char는 4바이트(32비트)이다.

    // casting - simple type change

    let my_number: u16 = 8;
    let second_number: u8 = 10;
    // 'as 타입' 을 이용하여 타입을 바꿀 수 있다.
    let _third_number = my_number as u8 + second_number;
    let _fourth_number = my_number + second_number as u16;

    let a = 'a' as u8; // char을 int로 바꾸면 아스키코드 값이 나온다.
    println!("My _a is {}", a); // 브라켓을 쓰면 문자열에 변수 값을 넣을 수 있다.
}