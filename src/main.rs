// Rust formatting

#[derive(Debug)]
struct Book {
    title: String,
    year: u16
}

fn main() {
    let my_book = Book {
        title: "Some title".to_string(),
        year: 1919
    };
    let book_2 = Book {
        title: "Book 2".to_string(),
        year: 2020
    };
    let t = 10;

    // Debug print format은 {변수명:?} 이런식으로 쓰면 된다
    // Python f"{string}" 처럼 쓰면 됨
    println!("Got books:{my_book:?}, {book_2:?}");
    

    // 아직은 아래 같이 프로퍼티를 쓰는건 지원 안함
    // 변수 그대로만 쓸 수 있음. 필요하면 변수를 하나 더 만들어야함
    // println!("My book name: {my_book.title}");

    // 정렬도 지원함
    println!("{t:ㅁ^15}");


    let width = 10;
    // 변수명$을 써서 변수로 정렬 길이를 정하는 것도 가능하다.
    println!("{t:ㅇ>width$?}");
}