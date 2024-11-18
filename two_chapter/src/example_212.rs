// example_212.rs
pub fn change_type_shadow () {
    let var = 1;
    println!("var={}", var);
    let var = String::from("기존 var를 섀도잉");
    println!("var={}", var);
}