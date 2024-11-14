// example_132.rs
pub fn create_string() {
    let mut s = String::from("Hello"); // Hello로 String 생성
    println!("{}", s);
    s.push_str(" Rust!"); // s 뒤에 Rust! 추가
    println!("{}", s);
}