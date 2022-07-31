// copy types
// copy 타입은 값을 복사하기만 하기에 값에 대한 소유권을 빼앗지 않는다.
// copy 타입을 많이 만들고 복사를 한다고 해도 메모리 양이 줄거나 하진 않는다.
fn prints_number(number: i32){
    println!("{}", number);
}

// String은 copy 타입이 아니다
// copy 타입 - 하나의 값을 변수들이 봐라봄
// clone 타입 - 값을 새로 메모리에 할당함
// clone도 마찬가지로 copy처럼 소유권을 빼앗기지 않는다.
// 그러나 값을 대하는 방식에 차이가 있다.
fn prints_string(input: String) {
    println!("{}", input);
}

fn main() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);

    let my_country = "Korea".to_string();
    prints_string(my_country.clone());
    prints_string(my_country);
}