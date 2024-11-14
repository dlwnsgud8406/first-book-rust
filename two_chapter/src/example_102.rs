pub fn exceed_scope_data_type() {
    let mut a: i8 = i8::MAX;
    a = a + 1; // MAX 초과
    println!("a = {}", a);
}