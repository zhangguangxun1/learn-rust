#[test]
fn test_use_debug() {
    // 所有的类型，若想用 std::fmt 的格式化打印，都要求实现至少一个可打印的 traits。
    // 仅有一些类型提供了自动实现，比如 std 库中的类型。所有其他类型都必须手动实现。
    //
    // fmt::Debug 这个 trait 使这项工作变得相当简单。所有类型都能推导（derive，即自动创建）fmt::Debug 的实现
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Simple(i32);

    println!("Default fmt::Debug -> {:?}", Simple(10));

    // 手动实现 fmt::Display 来做到。fmt::Display 采用 {} 标记
    // 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
    impl std::fmt::Display for Simple {
        // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "impl fmt::Display -> {}", self.0)
        }
    }

    print!("{}", Simple(1));
}

#[test]
fn test_impl_display() {
    // 对一个结构体实现 fmt::Display，其中的元素需要一个接一个地处理到，这可能会很麻烦。
    // 问题在于每个 write! 都要生成一个 fmt::Result。
    // 正确的实现需要处理所有的 Result。
    // Rust 专门为解决这个问题提供了 ? 操作符
    struct List(Vec<i32>);

    impl std::fmt::Display for List {
        // 中间用 ? 是发生错误直接返回, 否则要一直处理错误信息, 最后一行不需要是本身就需要这个返回类型
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            // count 迭代次数, 就是数组下标 不知道为啥不叫 index 一眼懂
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
            }
            writeln!(f, "]")
        }
    }

    println!("{}", List(vec![1, 2, 3]));
}
