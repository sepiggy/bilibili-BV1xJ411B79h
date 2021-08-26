const MAX_POINT: u32 = 100000;

fn main() {
    // 可变性
    let a = 1;
    let mut b: u32 = 2;
    println!("a={}", a);
    println!("b={}", b);
    b = 3;
    println!("b={}", b);

    // 隐藏性
    let b: f32 = 1.1;
    println!("b={}", b);
    
    // 常量
    println!("MAX_POINT={}", MAX_POINT);
}
