fn main() {
    let mut s = String::from("hello");
    change(&mut s); // 创建一个可变引用 &mut s
    println!("{}", s);

    // 可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用。
    let r1 = &mut s;
    let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time
    // 这个限制的好处是 Rust 可以在编译时就避免数据竞争。

    // 可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有：
    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;

    // 类似的规则也存在于同时使用可变与不可变引用中。这些代码会导致一个错误：
    // 不能在拥有不可变引用的同时拥有可变引用。
    let r1 = &s; // no problem
    let r2 = &s; // no problem, 多个不可变引用是可以的
    let r3 = &mut s; // BIG PROBLEM, cannot borrow `s` as mutable because it is also borrowed as immutable

    /* 引用的规则
       * 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
       * 引用必须总是有效。
    */
}

fn change(some_string: &mut String) { // 接受一个可变引用 some_string: &mut String
    some_string.push_str(", world");
}