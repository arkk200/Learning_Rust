// struct = and
// enum = or (선택)

// enum은 enumeration을 줄인말이다.
// e = from, number

enum ThingsInTheSky { // enum도 struct처럼 대문자로 시작한다.
    Sun,
    Starts,
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        // enum의 요소 접근은 'ENUM::요소' 형식으로 쓰면 된다.
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Starts
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun"),
        ThingsInTheSky::Starts => println!("I can see the starts")
    }
}

fn main() {
    let time = 20;
    let sky_state = create_skystate(time);
    check_skystate(&sky_state);

    // 한 번에 한 줄로 줄임
    check_skystate(&create_skystate(8));
}