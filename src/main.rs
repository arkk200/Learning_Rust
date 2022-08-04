use std::collections::HashMap;

fn main() {
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        // HashMap은 key, value 둘 다 있어야 한다.
        city_hashmap.insert(city, "Canada");
    }

    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }

    println!("{:?}", city_hashmap["Bielefeld"]);

    // 좀 더 안전하게 하려면 HashMap 역시 .get()을 쓰면된다.
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Doesn't include this"));

    
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Noderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");
    // 최종적으로 "Eye of the World"가 덮어 씀

    // hashmap에서 키를 가져올 땐 값을 빌려주어야 한다.
    println!("{:?}", book_hashmap.get(&1));

    if let Some(book_name) = book_hashmap.get(&1) {
        println!("Already got a book: \"{}\"", book_name);
    } else {
        book_hashmap.insert(1, "Le Petit Price");
    }
}