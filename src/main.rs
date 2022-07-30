fn main() {
    /* String = Sized type
    크기를 마음대로 늘리거나 줄일 수 있음
    참조를 하지 않아 자기의 데이터를 가지고 있음
    변수가 없어지면 데이터가 없어짐
    관리하기 편함
    데이터가 힙 영역에 있음 -> 컴파일러가 데이터가 얼마나 큰지 알 수 있음
    */
    /* &str
    참조를 함, 자기 데이터를 갖지 않음, 크기를 마음대로 조절 못함
    참조를 하기에 참조를 하는 데이터가 사라지면 안전하지가 않음
    그렇기에 참조하고 있는 데이터가 삭제되는 것 까지 고려해야함
    편리하지 않음
    &을 쓰는 까닭은 그냥 str은 문자열의 크기를 알 수 없다.
    따라서 참조를 통해 값을 가져온다.
    */
    let mut _str1 = "&str"; // 그러나 문자열의 기본 타입은 &str이다
    /*
    _str1.pop();
    _str1.push('!');
    &str 타입은 위 방법들을 쓸 수 없다.
    */
    let _str2 = "String1".to_string(); // String
    /* .to_string() 을 쓰면 String이 된다.
    이 외에도 다양한 방법이 있다.
    */
    let _str3 = String::from("String2"); // String
    // growable + shrinkable이 가능하다.
    let mut my_other_name = "String3".to_string(); // String
    my_other_name.pop();
    my_other_name.push('!');
    println!("{}", my_other_name);
}