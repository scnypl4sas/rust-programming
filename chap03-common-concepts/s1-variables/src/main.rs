fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // ******************************************************************************************
    // 标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

    // 整型
    // Rust 常量的命名规范是使用下划线分隔的大写字母单词
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "______";
    println!("{}", spaces);
    let spaces = spaces.len();
    println!("The length is: {}", spaces);
}

