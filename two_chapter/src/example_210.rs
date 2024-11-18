// example_210.rs
pub fn violate_mul () {
    let var = 1;
    println!("var={}", var);
    var = var + 1; // 컴파일 오류 발생
    println!("var={}", var);
}