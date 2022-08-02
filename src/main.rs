// Collectoin types
// Array 배열: []

fn main() {
    let array1 = [1, 2, 3];
    // let array2 = ["One", "Two"]; // 타입: [&str; 2]
    let array3 = ["One", "Two", "Five"]; // 타입: [&str; 3]
    let array4 = ["One", "Two", "Five"];
    println!("{}", array1[0]);
    // Rust에서 배열은 크기에 따라서도 다른 타입이 된다.
    // println!{"Is array the same as array2? {}", array2 == array3};
    // 배열 추가하거나, 삭제하거나 서로 비교할 수 없다.
    println!{"Is array the same as array2? {}", array4 == array3};
    // 배열의 타입을 알 수 있는 방법은 배열명.아무문잠ㅈㄷㄹㄴㅇㄻㅈㅊ(); 를 치면 된다.
    // array3.wehfbaiwef(); // 그러면 오류가 뜨는데 그 때 확인할 수 있다.

    // 배열은 버퍼를 만들 때 많이 쓴다.
    let array = [0; 640]; // 0이 640개 채워져있는 배열 생성
    println!("{:?}", array);
    // 버퍼로 만들려면 배열에 mut를 달아야 한다.
    let mut _array = [0; 640];
    
    // 월, 일 같이 수정되거나, 삭제, 추가되면 안되는 배열은 배열의 성질이 추가, 삭제가 안되는 것이므로 그냥 쓰면 된다.
    let array = ['2', '1'];
    // 문자나 문자열 배열의 요소를 디버그 프린트하면 따옴표까지 같이 출력된다.
    println!("{}", array[0]); // Rust도 배열의 첫번째가 0부터 시작한다.
    // println!("{}", array[2]); // 존재하지 않는 요소를 출력하려고 하면 에러가 난다.
    // 위 코드를 안전하게 짜려면 배열.get(인덱스)를 써주면 된다.
    println!("{:?}", array.get(3));
    // .get()은 값이 존재하면 Some, 없으면 None을 출력한다.
}