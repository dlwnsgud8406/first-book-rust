// example_111.rs
pub fn direct_tuple_refer() {
    let (x, y, z) : (i32, char, bool) = (1, 'a', true);
    println!("x={}, y={}, z={}", x, y, z);
}