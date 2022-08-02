// loops

fn main() {
    let mut counter = 0;
    let mut counter2 = 0;

    'first_loop: loop { // '루프명: loop 로 이름을 지정해줄 수 있다.
        counter += 1;
        println!("The counter is: {}", counter);
        if counter > 9 {
            println!("Now entering the second loop");

            'second_loop: loop {
                println!("The seconde counter is: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop; // break 옆에 지정한 루프명을 써서 원하는 루프를 break 시킬 수 있다.
                }
            }
        }
    }
}