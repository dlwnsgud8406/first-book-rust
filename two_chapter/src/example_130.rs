// example_130.rs
pub fn string_slicing_to_str() {
    // 문자열 리터럴을 사용해 s 생성
    let s: &str = "Hello 러스트!";

    // s 값 출력
    println!("문자열: {}", s);

    // 문자열 슬라이싱
    let slice: &str = &s[0..5];
    println!("슬라이스: {}", slice);
}