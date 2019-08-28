fn main() {
    let number = 7;

    // 条件 必须 是 bool 值
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = false;
    let number = if condition {
        5
    } else {
        6 // if 的每个分支的可能的返回值都必须是相同类型, 如果是 "six" 会报错 error[E0308]: if and else have incompatible types
    };
    println!("The value of number is: {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // loop 的一个用例是重试可能会失败的操作，比如检查线程是否完成了任务。
                               // 循环停止后仍然需要某个值，可以将该值放在 break 表达式后，以便在循环外使用。
        }
    };
    assert_eq!(result, 20);

    let mut number = 3;
    while number != 0 { 
        println!("{}", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4) {
        println!("{}", number);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }

    println!("LIFTOFF!!!");
}
