fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。因为并不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃。
                                     
    println!("The length of '{}' is {}.", s1, len);
    change(&s1);
}

fn calculate_length(s: &String) -> usize { // 函数签名使用 & 来表明参数 s 的类型是一个引用。
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生
  // 变量 s 有效的作用域与函数参数的作用域一样，不过当引用离开作用域后并不丢弃它指向的数据，因为我们没有所有权。
  /* 
     当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。
     我们将获取引用作为函数参数称为 借用（borrowing）。
  */

// 尝试修改借用的变量
fn change(some_string: &String) {
    some_string.push_str(", world"); // cannot borrow immutable borrowed content `*some_string` as mutable
}
// 正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。