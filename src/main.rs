// trait

// From, Into

fn main() {
    // from이 있으면 into도 쓸 수 있음
    let my_name = String::from("lmj");
    let my_city:String = "Busan".into();

    println!("{} {}", my_name, my_city);
    // String도 마찬가지로 변수명.아무글잠ㅈㄷㄻㅈ() 식으로 치면 타입을 볼 수 있음
    // my_city.aweiuvbwscewviabwe();
    let _my_vec: Vec<i32> = Vec::from([8, 9, 10]); // Vec은 from trait가 있음
}