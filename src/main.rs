use std::mem::size_of_val;

// 원래 크기는 1+1+1+4로 7바이트여야 하는데
// alignment가 자동으로 8바이트로 맞춰줌
struct Numbers {
    one: u8,
    two: u8,
    three: u8,
    four: u32
}

#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
    
    // 갑자기 말도 안되게 큰 크기의 요소가 있으면
    // 이걸 다른 struct에 옮겨서 따로 쓰는게 좋다
    all_populations: [u32; 5500]
}

fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justing Trudeau".to_string();

    let my_country = Country {
        // 속성명과 속성 값을 주는 변수 이름이 같을 경우
        // 이런식으로 세미콜론과 변수이름을 생략할 수 있다.
        population,
        capital,
        leader_name,
        all_populations: [500; 5500]
    };
    println!("Country is {} bytes in size", size_of_val(&my_country));

    let numbers = Numbers {
        one: 8,
        two: 19,
        three: 20,
        four: 30
    };
    println!("Size is: {}", size_of_val(&numbers));
}