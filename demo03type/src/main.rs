fn main() {
    // bool
    let is_true = true;
    println!("is_true={}", is_true);
    let is_false = false;
    println!("is_false={}", is_false);

    // char
    // char在rust里是32位的,区别c++
    let a = 'a';
    println!("a={}", a);

    let b = '你';
    println!("b={}", b);

    // int
    let c: i8 = -111;
    println!("c={}", c);
    let d = 0.0009;
    println!("d={}", d);

    // 自适应类型
    // isize, usize
    println!("max={}", usize::max_value());

    // 数组
    // "[type; size]"整体作为数组类型, size也是数组类型的一部分
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0]={}", arr[0]);

    let arr1 = [1, 2, 3];
    show(arr1);

    // 元组
    let tup: (i32, f32, char) = (-3, 3.69, '好');
    let tup = (-3, 3.69, '好');
    println!("{},{},{}", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("{},{},{}", x, y, z);
}

fn show(arr: [u32; 3]) {
    println!("------------------------");
    for i in &arr {
        println!("{}", i);
    }
    println!("------------------------");
}
