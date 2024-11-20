pub fn anonymous() {
    let x = 1;
    let y = 2;
    let ret = { // 익명함수의 반환값을 ret에 저장한다.
        x + y // 세미콜론이 없다.
    }; // 세미콜론이 필요하다.

    println!("{}+{}={}", x, y, ret);
}