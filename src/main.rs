// Dereference and the dot operator

struct Item {
    number: u8
}

impl Item {
    fn compare_number(&self, other_number: u8) {
        println!("Are they equal? {}", self.number == other_number);
    }
}

fn main() {
    let my_number = 10;
    let _reference = &my_number;

    // i32와 &i32는 타입이 달라 에러가 남
    // println!("Are they the same? {}", my_number == _reference);

    let item = Item {
        number: 10
    };

    let _reference = &item.number; // &u8
    // 얘 역시도 에러가 남
    // println!("{}", _reference == 10);


    let reference_item = &item; // &item
    let other_reference_item = &reference_item; // &&item

    // .연산자를 쓰면 러스트가 알아서 dereferencing 함 (*을 쓸 필여가 없음)
    reference_item.compare_number(10);
    other_reference_item.compare_number(10);
}