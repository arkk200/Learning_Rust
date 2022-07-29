fn main() {
    let my_city = "Busan";
    let year = 2022;
    let population = 3429000;
    println!("The city of {} in {} had a population of {}", my_city, year, population);
    /*
    println!("The city of {my_city} in {year} had a population of {population}", my_city, year, population);
    현재 버전(1.62.1)에선 괄호 안에 '"{변수명}", 변수명' 형식으론 쓸 수 없다.
    대신 '"{이름}", 이름 = 변수명' 형식으로 대체할 수 있다.
     */
    // "{이름}", 이름 = 변수명
    println!("The city of {city} in {year} had a population of {population}",
        city = my_city,
        year = year,
        population = population
    );
    /* '"{이름}", 이름 = 변수명' 형식 외에 {} 안에 숫자를 넣어 변수 값을 가지고 올 수 있다. 0부터 순서대로 시작한다.*/
    println!("The city of {0} in {1} had a population of {2}, I live in {0}", my_city, year, population);
    // 이런걸 문자열 보간(string interpolation)이라고 한다.
}