// example_123.rs

use std::io; // std::io 패키지 로드

pub fn runtime_check_bof() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("숫자를 입력해주세요.");
    let mut read = String::new(); // 입력값을 저장할 문자열 데이터 생성
    io::stdin().read_line(&mut read).unwrap(); // 키보드 입력을 읽습니다.
    let index: i32 = read.trim().parse().unwrap(); // 문자열을 숫자로 변환합니다.

    println!("arr[{}]={}", index, arr[index as usize]);
}