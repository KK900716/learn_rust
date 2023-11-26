
pub fn _basic_data_type() {
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

pub fn _basic_controller() {
    {
        let _x = if true {
            2
        } else {
            1
        };
    }
    {
        println!("{}", match 1 {
            1 => {
                1
            }
            _ => {
                2
            }
        })
    }
    {
        let mut x = 1;
        loop {
            println!("{}", x);
            x += 1;
            if x == 10 {
                break;
            }
        }
    }
    for _ in 1..10 {
        println!("a")
    }
}

pub fn _basic_std() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("exception");
    println!("{}", input)
}

pub fn _inner_basic_err() -> Result<(), String> {
    return Err(String::from("内"));
}

pub fn basic_err() {
    _inner_basic_err().expect("外");
}