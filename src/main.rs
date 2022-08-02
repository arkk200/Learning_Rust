fn main() {
    let sky = "cloudy";
    let temperature = "warm";

    // 변수 여러개를 튜플로 감싸주면 여러개를 비교할 수 있다.
    match (sky, temperature) {

        // match는 여러 조건 중 하나만 실행한다.

        // 밑에 조건이 있는데도 _를 먼저 쓰면 잘못 쓴거라고 컴파일러가 경고한다.
        // 튜플 안에 요소도 마찬가지로 _를 보고 경고를 한다.
        // _ => println!("Not sure what the weather is..."), // _ 는 마지막에 씀
        // ("cloudy", _) => println!("Cloudy and something else"), 
        ("cloudy", "cold") => println!("It's not very nice today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", _) => println!("Cloudy and something else"),
        _ => println!("Not sure what the weather is..."),
    }

    let children = 5;
    let married = true;

    match (children, married)  {
        // 패턴은 match옆에 괄호에서 정해졌으므로 튜플안에있는 변수명는 꼭 변수 이름과 같게 쓸 필요가 없다.
        (children, married) if !married => println!("Not married with {} children", children),
        (c, m) if c == 0 && m => println!("Married but with no children"),
        _ => println!("Some other type of marriage and children combination")
    }
}