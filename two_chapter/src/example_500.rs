pub fn fn_ex() {
    add(1, 2);
}

fn add(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, (x+y));
}