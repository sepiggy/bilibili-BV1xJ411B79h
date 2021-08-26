fn other_fun() {
    println!("this is a function");
}

fn other_fun1(a: i32, b: u32) {
    println!("a={}, b={}", a, b);
}

fn main() {
    other_fun();
    other_fun1(1, 2);
}
