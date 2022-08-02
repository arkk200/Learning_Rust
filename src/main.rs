// 러스트에는 struct가 3가지가 있다.
// unit struct - 아무것도 없는, 이름만 있는 struct
// unit struct는 바이트 크기가 0이다.
struct FileDirectory; // struct의 앞 글자는 무조건 대문자이다.

fn take_file_directory(_input: FileDirectory) { // 아무것도 없어도 쓸 수 있다.
    println!("I got a file directory");
}

// tuple struct
// #[derive(Debug)]
struct Colour (u8, u8, u8);

// name struct
#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String
}

fn main() {
    // unit struct (거의 안 씀)
    let x = FileDirectory;
    println!("The size is {}", std::mem::size_of_val(&x));
    take_file_directory(x);

    println!("");

    // tuple struct
    let my_colour = Colour(20, 50, 100);
    println!("The second colour is {}", my_colour.1); // 튜플은 프로퍼티를 이용해서 요소를 가져옴
    // println!("The second colour is {:?}", my_colour);

    println!("");

    // name struct
    let canada = Country {
        population: 35_000_000,
        capital: "Ottawa".to_string(),
        leader_name: "Justin Trudeau".to_string()
    };
    println!("The population is: {}\nThe capital is: {}\nLeader name is: {}", canada.population, canada.capital, canada.leader_name);
    println!("The country is: {:#?}", canada); // Debug print에 #을 사이에 넣으면 깔끔하게 출력됨
}