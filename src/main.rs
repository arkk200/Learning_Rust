fn main() {
    let my_vec = vec![2, 3, 4];
    // .get은 배열에서 해당 인덱스에 값이 있으면 Some(값),
    // 없으면 None을 반환함
    let _get_one = my_vec.get(0);
    let _get_two = my_vec.get(20);
    println!("{:?}, {:?}", _get_one, _get_two);

    // 이를 활용해서 안전하게 원하는 값만 출력할 수 있다.
    for index in 0..10 {
        match my_vec.get(index) {
            Some(num) => println!("The number is: {}", num),
            None => {}
        }

        // if let을 이용하면
        /*
        let a = Some(0i32);
        match a {
            Some(3) => println!("three"),
            _ => (),
        } 를
        if let Some(3) = a {
            println!("three");
        } 로 줄일 수 있다. (비교를 할 때 = 을 하나만 씀)
        출처: https://rinthel.github.io/rust-lang-book-ko/ch06-03-if-let.html
        if let은 서로 패턴이 대응된다면 패턴식을 수행한 후 분기로 접어든다.
        */
        if let Some(num) = my_vec.get(index) {
            println!("The number is: {}", num);
        }
    }
    
    // if let과 마찬가지로 while let도 존재한다, while let도 마찬가지이다.
    
    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"]
    ];

    println!("");

    for mut city in weather_vec {
        println!("For the city of: {}", city[0]);
        // vec!["Berlin", "cloudy", ...]에서 하나씩 pop()을 하고
        // 안에 있던 문자열을 반환함 (pop()은 뒤에서부터 하나씩 요소를 제거함)
        while let Some(information) = city.pop() {
            // information.parse를 할 때 타입이 i32인게 나오면
            // Ok(_)의 패턴식과 같아지고 패턴식을 실행함
            if let Ok(number) = information.parse::<i32>() { // turbofish ::<>
                println!("The number is: {}", number);
            }
        }
    }
}