use std::collections::{HashMap, BTreeMap};

// HashMap and BTreeMap

// HashMap은 파이썬의 딕셔너리, JS의 객체와 비슷하다.
// key가 있고 value가 있다.

// BTreeMap도 HashMap처럼 key와 value가 있지만
// 다른 점은 순서를 가지고 있다.

struct City {
    name: String,

    // 각각 year(key), population(value) 이다.
    population: HashMap<u32, u32>
}

struct AnotherCity {
    name: String,

    // 각각 year(key), population(value) 이다.
    population: BTreeMap<u32, u32>
}

fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new()
    };

    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 123);
    // 이미 HashMap에 같은 이름의 키가 있을 경우 덮어쓴다.
    tallinn.population.insert(2020, 437_619);

    // 실행할 때마다 순서가 섞여서 출력됨, HashMap은 성능 때문에 순서가 없음
    for (year, population) in tallinn.population {
        println!("In the year {} the population was {}", year, population);
    }

    for _i in 0..3 {println!("");}


    // BTreeMap
    let mut tallinn = AnotherCity {
        name: "Tallinn".to_string(),
        population: BTreeMap::new()
    };

    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 123);
    // BTreeMap도 마찬가지로 같은 이름의 키가 있을 경우 덮어쓴다.
    tallinn.population.insert(2020, 437_619);

    // key값이 오름차순으로 정렬되어 있는 것을 확인할 수 있다.
    for (year, population) in tallinn.population {
        println!("In the year {} the population was {}", year, population);
    }
}