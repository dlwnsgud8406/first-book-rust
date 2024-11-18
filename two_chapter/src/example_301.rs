// example_301.rs
pub fn let_if() {
    let condition = true;
    let ret = if condition == true {
        String::from("조건이 참입니다.")
    }
    else {
        String::from("조건이 거짓입니다.")
    };
    println!("ret={}", ret);
}