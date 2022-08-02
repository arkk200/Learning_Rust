// use: 함수 같은걸 줄여줌

enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*; // Mood::*;을 쓰면
    // Mood::Happy, Mood::Sleepy... 앞에 'Mood::'를 제거할 수 있음

    // match앞에 변수넣고 그 변수를 굳이 반환할 필요없이
    // match에서 return되는 값을 바로 return할 수 있다.
    match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2
    }
}

// -----------

enum Season {
    // enum은 컴파일러가 볼 때 숫자가 있다.
    Spring, // 0
    Summer, // 1
    Autumn, // 2
    Winter, // 3
}

fn main() {
    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {}", happiness_level);

    // -----------
    use Season::*;

    // Vec<Season> 타입
    let four_seasons = vec![Spring, Summer, Autumn, Winter];

    for season in four_seasons {

        // as 정수로 하면 컴파일러가 인식하는 숫자가 출력된다.
        println!("The number is: {}", season as u32);
    }
}