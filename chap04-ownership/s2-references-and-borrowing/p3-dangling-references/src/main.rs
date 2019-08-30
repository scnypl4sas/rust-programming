fn main() {
    /* 
    在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。
    */
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn dangle() -> &String { // dangle 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串
    &s  // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
/* 
    因为 s 是在 dangle 函数内创建的，当 dangle 的代码执行完毕后，s 将被释放。
    不过我们尝试返回它的引用。这意味着这个引用会指向一个无效的 String
    解决方法是直接返回 String：
*/

// 解决方法是直接返回 String：
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
