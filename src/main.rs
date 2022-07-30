// OWNERSHIP
/* (1)-- move semantics 
    함수에서 인자를 받을 때 참조로 받은게 아니기에
    값에 대한 소유권을 가져간다. (변수 country는 값의 소유권을 뺏기게 됨)
    따라서 print_country() 함수에 한 번 더 country를 인자로 전달하면
    값이 이동됐다는 에러가 발생한다.
*/

fn print_country(country_name: String) {
    println!("My country is {}", country_name);
}
fn print_country_with_return(country_name: String) -> String{
    println!("My another country is {}", country_name);
    country_name // 이런식으로 값을 다시 반환해 줄 순 있으나 참조를 쓰는게 더 낫다
}

fn print_country_with_reference(country_name: &String){
    println!("My third country is {}", country_name);
}

fn main() {
    let country = "my country".to_string();
    print_country(country); // --(1)
    // print_country(country); // 에러가 발생함
    let mut another_country = "another country".to_string();
    // 값은 다시 돌려 받음
    another_country = print_country_with_return(another_country);
    println!("{}", another_country);
    let third_country = "third country".to_string();
    print_country_with_reference(&third_country);
    print_country_with_reference(&third_country);
    print_country_with_reference(&third_country);
}