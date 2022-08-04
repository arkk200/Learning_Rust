use std::collections::{HashMap, HashSet};

fn main() {
    let data = vec![
        ("female", 9),
        ("male", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];

    let mut survey_hashmap = HashMap::new();

    for (gender, num) in data { // (&strm i32)
        // 튜플로 이루어진 배열이므로 프로퍼티로 가져옴
        // 만약 해당 키가 없으면 Vec을 value로 만들고 push를 함
        // 있을 경우 Vec은 value로 다시 만들지 않고
        // 이미 만들어진 Vec에 push만 함
        survey_hashmap.entry(gender).or_insert(Vec::new()).push(num);
    }

    for (male_or_femail, nums) in survey_hashmap {
        println!("{:?}, {:?}", male_or_femail, nums);
    }


    // HashSet은 어떤 배열에 값이 있는지 없는지만 확인할 때 쓰면 된다.
    let many_numbers = vec![
        94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,
        51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,
        35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,
        96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,
        58, 64, 80, 16, 61, 57, 14, 11];

    // HashSet은 generic이 하나만 들어간다.
    let mut number_hashset = HashSet::new();
    
    for num in &many_numbers {
        number_hashset.insert(num);
    }

    // HashSet은 이름 그대로 집합으로 중복되는 원소는 하나로 없앤다.
    let hashset_length = number_hashset.len();
    println!("Total Length: {}, There are {} unique numbers, so we are missing {}.", &many_numbers.len(), hashset_length, 100 - hashset_length);

    let mut missing_vec = vec![];
    for number in 0..100 {
        if number_hashset.get(&number).is_none() {
            missing_vec.push(number);
        }
    }

    print!("It does not contain: ");
    for num in missing_vec {
        print!("{} ", num);
    }
}