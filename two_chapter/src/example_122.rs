// example_122.rs
pub fn check_bof() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; // i32 타입을 가지는 5개 원소
    arr[6] = 7; // 6번 인덱스에 값을 기입

    println!("arr[6]={}", arr[6]); // 6번 인덱스를 참조
}