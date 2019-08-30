fn main() {
    let s = String::from("hello world");
    let s1 = &s[0..5]; // or &s[0..=4], &s[..5]
    let s2 = &s[6..11]; // or &s[6..=10], &s[6..]。&s[..] 获取整个字符串的 slice
    println!("{}, {}", s1, s2);

    let mut s = String::from("hello world");
    let word = first_word(&s); // 获取一个不可变引用

    // s.clear(); // 获取一个可变引用
    // error! cannot borrow `s` as mutable because it is also borrowed as immutable
    // 当拥有某值的不可变引用时，就不能再获取一个可变引用。因为 clear 需要清空 String，它尝试获取一个可变引用，它失败了。

    println!("the first word is: {}", word);

    /*----------------------------------------------------------- */

    let my_string = String::from("hello world");

    // first_word 中传入 `String` 的 slice
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);

     /*----------------------------------------------------------- */
     let a = [1, 2, 3, 4, 5];
     let slice = &a[1..3];
     // 这个 slice 的类型是 &[i32]。它跟字符串 slice 的工作方式一样，通过存储第一个集合元素的引用和一个集合总长度。
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}