use std::collections::VecDeque;

// VecDeque
// Deque(double - ended queue)는
// 양방향으로 값을 집어넣을 수 있는 queue를 의미한다.

fn main() {
    // Vec도 .remove(0)을 써서 앞에 값을 빼올 수 있다.
    let mut _my_vec = vec![8, 9, 10];
    println!("{}, {:?}", _my_vec.remove(0), _my_vec);
    // remove는 값을 제거하면 뒤에서부터 값들을 한칸씩 앞으로 당긴다.
    // 만약 배열 길이가 길어진다면 성능이 떨어진다는 단점이 있기에 이럴 땐 Deque를 사용한다.
    let mut _very_larget_vec = vec![0; 600_000];

    /*
    for _ in 0..600_000 { // 60만 번 값을 제거하면서 1800억번정도 값을 복사하고 앞 인덱스에 붙여넣음
        _very_larget_vec.remove(0);
    }
    */

    // VecDeque
    let mut _very_larget_vecdeque = VecDeque::from(vec![0; 600_000]);
    for _ in 0..600_000 { // 60만 번 정도 pop()하기만 함
        // push_front(), pop_front()로 앞에서부터 값을 넣거나 제거하고
        // push_back(), pop_back()으로 뒤에서부터 값을 넣거나 제거함
        _very_larget_vecdeque.pop_front();
    }
}