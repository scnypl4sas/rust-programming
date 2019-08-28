fn main() {
    println!("Hello, world!");
    another_function();
    parameter_function(5);
    multiparam_function(5, 6);
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // 表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// Rust 不关心函数定义于何处，只要定义了就行。
fn another_function() {
    println!("Another function.");
}

// 函数参数
fn parameter_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn multiparam_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// 包含语句和表达式的函数体
// Rust 是一门基于表达式（expression-based）的语言
// 语句（Statements）是执行一些操作但不返回值的指令。表达式（Expressions）计算并产生一个值。

// 具有返回值的函数
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 行尾加上一个分号，把它从表达式变成语句，我们将看到错误 “mismatched types”。
}