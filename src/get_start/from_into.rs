use std::convert::From;

/// From 和 Into 两个 trait 是内部相关联的，实际上这是它们实现的一部分。
/// 如果我们能够从类型 B 得到类型 A，那么很容易相信我们也能够把类型 B 转换为类型 A。
/// From Into 并不是反向转换, 比如 我实现了一个从 String 数字 转为 i32 的数字,
/// Rust 的意思是 From 可以将 Sting -> i32, 如果你实现了 From 那么自动就获得了 Into, Into 也一样能将 String -> i32
/// 而不是那种我从 String -> i32 用 From, 我从 i32 -> String 用 Into, 哈哈哈
/// 注意 B -> A 是单向的, 不过这个 Into 就不是必要的了
///
/// 类似于 From 和 Into，TryFrom 和 TryInto 是类型转换的通用 trait。
/// 不同于 From/Into 的是，TryFrom 和 TryInto trait 用于易出错的转换，也正因如此，其返回值是 Result 型。
///
///
/// 要把任何类型转换成 String，只需要实现那个类型的 ToString trait。
/// 然而不要直接这么做，您应该实现fmt::Display trait，它会自动提供 ToString，并且还可以用来打印类型
///
/// 我们经常需要把字符串转成数字。完成这项工作的标准手段是用 parse 函数。
/// 我们得提供要转换到的类型，这可以通过不使用类型推断，或者用 “涡轮鱼” 语法（turbo fish，<>）实现。
///
/// 只要对目标类型实现了 FromStr trait，就可以用 parse 把字符串转换成目标类型。
/// 标准库中已经给无数种类型实现了 FromStr。如果要转换到用户定义类型，只要手动实现 FromStr 就行。
///

#[allow(dead_code)]
#[derive(Debug)]
struct Number(i32);

// 从一个字符串数字生成 Number
impl From<&str> for Number {
    fn from(s: &str) -> Self {
        Self(s.parse().unwrap_or(0))
    }
}

#[test]
fn test_from() {
    // From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，因此它提供了一种类型转换的简单机制。
    // 在标准库中有无数 From 的实现，规定原生类型及其他常见类型的转换功能

    let simple_str: &str = "hello";
    let simple_string = simple_str.to_string();
    let simple_string_2 = String::from(simple_str);
    println!(
        "simple_str: {}, simple_string_2: {}",
        simple_string, simple_string_2
    );

    let num1 = Number::from("10");
    println!("{:?}", num1);
}

#[test]
fn test_into() {
    // Into trait 就是把 From trait 倒过来而已。
    // 也就是说，如果你为你的类型实现了 From，那么同时你也就免费获得了 Into

    // 使用 Into trait 通常要求指明要转换到的类型，因为编译器大多数时候不能推断它。
    // 不过考虑到我们免费获得了 Into，这点代价不值一提

    let simple_str: &str = "100";
    let num: Number = simple_str.into();
    println!("{:?}", num);
}
