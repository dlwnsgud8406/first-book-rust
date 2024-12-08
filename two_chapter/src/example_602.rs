pub fn move_example() {
    let s = String::from("Hello");
    let f = move || { // move 클로저는 소유권을 이전한다.
        println!("s: {}", s) // 여기서 s의 소유권을 가진다.
    };
    f();

    println!("s: {}", s); // 소유권으로 인한 컴파일 오류
}