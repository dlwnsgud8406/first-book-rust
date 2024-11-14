// example_120.rs
pub fn first_array() {
    // 정수 배열 arr을 선언하고 [1, 2, 3, 4, 5]로 초기화한다.
    let arr = [1, 2, 3, 4, 5];

    // 배열 arr의 각 요소에 대해 반복한다.
    for a in arr{
        // 현재 요소의 값을 콘솔에 출력한다.(얘는 줄바꿈 X)
        print!("{}, ", a);
    }
    // 얘는 줄바꿈 O
    println!("");
}