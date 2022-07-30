// OWNERSHIP - 소유권

/*
fn return_string() -> &String { // 이 함수는 존재할 수 없다.
    let country = String::from("대한민국2");
    // country 변수는 이 함수 내에서만 쓰이는데
    //이 함수를 참조한 데이터를 보낸다해도
    // 함수가 끝나면 참조 당했던 변수(country)가
    // 사라지기에 안전하지 않기 때문이다.
    return &country
}
 */

// & = reference

fn main() {
    let country = String::from("대한민국");
    let _ref_one = &country;
    let _ref_two = &country; // 하나의 데이터에 여러개의 참조를 해도 된다.

    println!("Country is: {}", _ref_one);
    // let _my_country = return_string(); // 안전하지 않다
}