pub fn check_data_type() {
    let number = 30;
    let long_number: i64 = 123456789123456789;
    let real = 10.22;
    let hangul_char = '러';

    println!("32비트 정수: {}", number);
    println!("64비트 정수: {}", long_number);
    println!("32비트 실수: {}", real);
    println!("문자: {}", hangul_char);
}
