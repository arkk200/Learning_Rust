fn main() {
    print!("this"); // 줄바꿈을 안 만듦
    print!("this2");
    println!("line_this3"); // print!("line_this3\n")
    print!("this4");
    println!("\n-------------------------");
    print!("this"); // 줄바꿈을 안 만듦
    print!("this2");
    print!("line_this3\n");
    print!("this4");
    println!("\n-------------------------");
    // raw text: \n, \t같은 것도 그대로 출력함, r#"문자열"# 형식으로 씀
    println!(r#"c:\this_drive\new_drive"#);
    println!("\n-------------------------");
    // 러스트에선 문자열에 줄바꿈을 해주면 실제로 줄바꿈을 해서 출력해준다.
    println!(
"Test
Teest
Teeest"
    );
    let my_variable = &"9";
    println!("{:p}", my_variable); // :p는 메모리 주소를 출력해준다.
    let my_variable = &185;
    println!("{:o}", *my_variable); // :p는 메모리 주소를 출력해준다.
    /*
    :?, 디버그 출력
    :p, 메모리 주소 출력
    :x, :X, 16진수로 출력
    :b, 이진수 출력
    :o, 팔진수 출력
    */
    let title = "TODAY'S NEWS";
    // 중앙(^)으로 정렬, 30글자로 맞추고 양 옆을 ㅁ 으로 채움
    println!("{:ㅁ^30}", title);
    // 오른쪽(>)으로 정렬, 15글자로 맞추고 왼쪽을 띄어쓰기로 채움
    println!("{: >15}", title);
    // 왼쪽(<)으로 정렬, 20글자로 맞추고 오른쪽을 샍 으로 채움
    print!("{variable1:샍<20}", variable1 = title) // 마지막 코드는 끝에 세미콜론(;)을 넣을 필요없음
}