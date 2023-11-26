pub fn basic_data_type() {
    let x1: bool = true;
    let x2: i32 = 255;
    let x3: f32 = 1.1;
    let x4: f64 = 1.2;
    let x5: char = 'a';
    println!("{},{},{},{},{}.", x1, x2, x3, x4, x5);
    let x6: &str = "Hello, Rust!";
    let mut x7 = String::from("Hello");  // 可变字符串类型
    println!("{}.\n{}.", x6, x7);
    x7.push_str(", Rust!");  // 追加字符串
    println!("{}.", x7);
    let x8: (i32, f64, &str) = (42, 3.14, "hello");
    let (_, _, _) = x8;  // 解构元组
    println!("{:?}.", x8);
    let x9: [i32; 3] = [1, 2, 3];
    println!("{:?}.", x9);
}