// example_311.rs
pub fn let_match() {
    let var = 1;
    let ret = match var {
        1 => String::from("하나"),
        2 => String::from("둘"),
        _ => String::from("기타"),
    };
    println!("ret = {}", ret);
}