fn main() {
    /* String은 재할당(reallocation)을 한다,
    예를 들어 16바이트를 할당했는데 데이터가 그 크기보다 넘어서면
    재할당하여 바이트 크기를 두 배로 늘린다.
    (추측)
    바이트 크기가 두 배로 느는건 .push()나 .push_str() 함수가 호출된 후
    문자열의 크기가 기존 크기를 넘어설 때 늘어난다.
    값이 존재할 때 최소 값은 8바이트이다.
     */
    // method = .function // .을 통해서 호출할 수 있는 함수가 메서드이다.
    /* String methods 종류
    .capacity()
    .push()
    .push_str()
    .pop()
    */
    let mut _my_name = "".to_string();
    // .len() 값은 .capacity() 값을 초과할 수 없다.
    // 할당할 바이트 크기를 알면 처음부터 24바이트를 할당하면 재할당이 없다.
    // let mut _my_name = String::with_capacity(24);
    println!("Length is {} Capacity is: {}", _my_name.len(), _my_name.capacity()); // 
    _my_name.push_str("lmj");
    _my_name.push('!'); // 문자 추가
    println!("Length is {} Capacity is: {}", _my_name.len(), _my_name.capacity()); // lmj!
    _my_name.push_str(" and "); // 문자열 추가
    _my_name.push_str("I live in Seoul!!!!!!!");
    // _my_name.push('2'); // 두 배로 늘어남
    println!("Length is {} Capacity is: {}", _my_name.len(), _my_name.capacity()); // lmj! and I ...
    println!("My na is {}", _my_name);
}