pub fn closure() {
    let x = 10;
    let add = |y| x + y; // add는 클로저 함수가 된다.
    println!("10 + 5 = {}", add(5));
}