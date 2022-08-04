use std::collections::BinaryHeap;

fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
    let mut remainder_vec = vec![];
    for num in input {
        remainder_vec.push(*num);
    }
    remainder_vec
}

fn main() {
    let many_nums = vec![0, 5, 10, 15, 20, 25, 30];

    // BinaryHeap의 특징은, 순서는 랜덤이지만
    // 값들 중 가장 큰 값은 무조건 앞에 배치되어 있음
    let mut my_heap = BinaryHeap::new();

    for num in many_nums {
        my_heap.push(num);
    }

    // my_heap에 값이 있으면 반복함
    while let Some(num) = my_heap.pop() { // Heap이니깐 앞에서부터 pop함
        println!("Popped off {}, Remaining numbers are: {:?}", num, show_remainder(&my_heap));
    }

    
    let mut jobs = BinaryHeap::new();

    // ( 중요도, 내용 )
    // 튜플이 push가 될 경우 튜플 안에서
    // 앞에 있는 요소를 기준으로 위에서 말한 규칙을 정함
    jobs.push((100, "Write back to email from the CEO"));
    jobs.push((80, "Finish the report today"));
    jobs.push((5, "Watch some YouTube"));
    jobs.push((70, "Tell your team members thanks for always working hard"));
    jobs.push((30, "Plan who to hire next for the team"));

    while let Some((_, content)) = jobs.pop() {
        println!("You need to: {}", content);
    }
}