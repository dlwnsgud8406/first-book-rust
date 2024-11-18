// example_211.rs
pub fn not_violate_mul () {
    let var = 1;
    println!("var={}", var);
    let var = var + 1; // 컴파일 오류 발생
    println!("var={}", var);
}