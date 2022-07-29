// mutablility, 변하기 쉬운
// 러스트는 변수를 선언하면 바꿀 수 없다.
// 바꾸려면 mut 라는 키워드를 써야한다.

// shadowing, 같은 변수 이름을 다시 쓰는 것

fn double(input: i32) -> i32 {
    input * 2
}
fn triple(input: i32) -> i32 {
    input * 3
}

fn main() {
    // mutability
    let _my_number = 10;
    // _my_number = 9; // 바꿀 수 없다는 에러가 뜸
    let mut _mutable_my_number = 10;
    _mutable_my_number = 9; // 변수를 선언할 때 mut를 쓰면 변수를 수정할 수 있다.
    let _is_alone_possible: i32;
    _is_alone_possible = 9; /* 선언할 때 굳이 초기화 할 필요가 없다.
    나중에 최초 1회 할당만 해도 된다.
     */
    let mut _i_dont_want_to_be_changed = 10; /* mut를 했음에도 바꾸지 않는다면
    에러가 뜬다는데 VSC에선 에러가 뜨지 않는다. */

    // shadowing
    // let my_variable = 10;
    let my_variable = "My variable"; /* 이 줄 뒤에 나올 my_variable은
    다 "My variable" 값을 가리키게 된다.
    */
    println!("{}", my_variable);

    let x = 9;
    let x = double(x); // double() 안에 들어간 x는 바로 위의 x를 가르킨다.
    let x = triple(x); // triple() 안에 들어간 x도 마찬가지로 바로위의 x(double(x))를 가르킨다.
    println!("{}", x);

    // 러스트도 블럭에 따라 변수가 구분된다.
    let my_var = 9;
    {
        let my_var = "some string";
        println!("{}", my_var); // "some string"
    }
    println!("{}", my_var); // "9"
}