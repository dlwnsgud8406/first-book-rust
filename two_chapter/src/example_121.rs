// example_121.rs
pub fn direct_refer() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // i32 타입을 가지는 5개 원소

    for i in 0..arr.len() {
        print!("{}, ", arr[i]);
    }
    println!("");
}