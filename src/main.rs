use std::fmt::Debug;

// traits
// power superpower

struct  _ThingToAdd {
    first_thing: u32,
    seconde_thing: f32
}

fn _print_as_debug<T>(input: T)
where
    T: Debug // 이러한 Debug가 trait이다
{
    println!("{input:?}");
}

fn main() {
    let my_thing = _ThingToAdd {
        first_thing: 32,
        seconde_thing: 8.8
    };

    let second_thing = _ThingToAdd {
        first_thing: 32,
        seconde_thing: 8.8
    };

    // 이런식으로 더하려고 할 때도 trait을 쓴다.
    // let sum = my_thing + second_thingl
}