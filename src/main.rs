fn add_is_great(country_name: &mut String) {
    country_name.push_str(" is great");
    println!("Now it says: {}", country_name);
}

// (1)-- immutable 변수를 인자로 받았는데 가능한 이유는
// 값으로 받있기에 값에 대한 소유권을 mut country name가 뺏게 되고
// 그렇기 때문에 바꿀 수 있게 된다.
fn add_is_great_with_mut(mut country_name: String){
    country_name.push_str(" is great");
    println!("Now it says: {}", country_name);
}

fn add_is_great_with_mut_and_return(mut country_name: String) -> String {
    country_name.push_str(" is great");
    println!("Now it says: {}", country_name);
    country_name
}

fn main() {
    let mut my_country = "country".to_string();
    add_is_great(&mut my_country);
    add_is_great(&mut my_country);

    let my_second_country = "seconde country".to_string();
    add_is_great_with_mut(my_second_country); // --(1)
    // println!("{}", my_second_country); // 값을 빼앗겼기에 에러가 뜸
    let mut my_third_country = "third country".to_string();
    // return을 이용해서 값을 다시 받음
    my_third_country = add_is_great_with_mut_and_return(my_third_country);
    println!("{}", my_third_country);
}