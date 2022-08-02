fn main() {
    let my_number = 5;
    let my_second_number = 10;

    // 중괄호를 쓰면 에러가 안나지만 컴파일러가 필요없다고 경고한다.
    if /*(*/my_number == 5/*)*/ && my_second_number == 10{
        println!("They both match");
    
    // 다른 언어와 마찬가지로 &&: and, ||: or을 지원한다.
    } else if my_number == 6 {
        println!("It's six");

    } else { // 7과 6이 아니니 It's a different number이 출력된다.
        println!("It's a different number");
    }

    // match
    let my_number = 5;

    // 다른 언어에서는 비슷한 switch가 있는데 match가 더 좋음
    // rust가 expression-based language인 만큼 match앞에 변수선언이 가능함
    let second_number = match my_number {
        0 => 23,
        1 => 56,

        // 모든 가능성을 다 적어주어야 안전하기에 _를 쓴다.
        _ => 17 // _는 i don't care || anything else 라는 뜻
    };
    println!("The second number is: {}", second_number);
}