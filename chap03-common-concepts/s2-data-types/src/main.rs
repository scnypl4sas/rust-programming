fn main() {
    // u32: 32 比特位的无符号整数。u8, u16, u32, u64, usize: ：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。
    // i32: 32 比特位的有符号整数。i8, i16, i32, i64, isize: ：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。
    let guess: u32 = "42".parse().expect("Not a number!"); // 如果不加类型注解 u32 会报错
    println!("The guess is: {}", guess);

    // 浮点型
    let f_x = 2.0; // f64
    let f_y = 3.0; // f32
    println!("f_x is: {}, f_y is: {}", f_x, f_y);

    // 数值运算
    let sum = 5 + 10; // 加法
    let difference = 95.5 - 4.3; // 减法
    let product = 4 * 30; // 乘法
    let quotient = 56.7 / 32.2; // 除法
    let remainder = 43 % 5; // 取余
    println!("{}, {}, {}, {}, {}", sum, difference, product, quotient, remainder);

    // 布尔型
    let t = true;
    let f: bool = false; // 显式指定类型注解
    println!("{}, {}", t, f);

    // 字符类型
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}", c, z, heart_eyed_cat);
    // Rust 的 char 类型代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容。
    // 在 Rust 中，拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值。
    // Unicode 标量值包含从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 在内的值。
    
    // ******************************************************************************************
    // 复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

    // 元组类型: 将多个其他类型的值组合进一个复合类型的主要方式。

    // tup 变量绑定到整个元组上，因为元组是一个单独的复合元素。为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // 使用模式匹配（pattern matching）来解构（destructure）元组值
    println!("{}, {}, {}", x, y, z);

    let tup = (500, 6.4, 1);
    // 除了使用模式匹配解构外，也可以使用点号（.）后跟值的索引来直接访问
    let first = tup.0;
    let second = tup.1;
    let third = tup.2;
    println!("{}, {}, {}", first, second, third);

    // 数组类型: 与元组不同，数组中的每个元素的类型必须相同。
    // Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小。
    let numarray = [1, 2, 3, 4, 5]; // let a: [i32; 5] = [1, 2, 3, 4, 5]; 
    println!("{}", numarray[2]);
    // 当你想要在栈（stack）而不是在堆（heap）上为数据分配空间（第四章将讨论栈与堆的更多内容），或者是想要确保总是有固定数量的元素时，数组非常有用。
    // 数组并不如 vector 类型灵活。vector 类型是标准库提供的一个 允许 增长和缩小长度的类似数组的集合类型。当不确定是应该使用数组还是 vector 的时候，你可能应该使用 vector。
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    println!("{}", months[6]);

    // 无效的数组元素访问
    // println!("{}", numarray[10]); 可以编译不过在运行时会因错误而退出
    // 在很多底层语言中，并没有进行这类检查，这样当提供了一个不正确的索引时，就会访问无效的内存。通过立即退出而不是允许内存访问并继续执行，Rust 让你避开此类错误。
}
