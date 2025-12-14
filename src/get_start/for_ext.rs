///
/// for in 结构能以几种方式与 Iterator 互动。
/// 在 迭代器 trait 一节将会谈到，如果没有特别指定，for 循环会对给出的集合应用 into_iter 函数，把它转换成一个迭代器。
/// 这并不是把集合变成迭代器的唯一方法，其他的方法有 iter 和iter_mut 函数。
///

#[test]
fn test_iter() {
    // iter - 在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍可以使用
    let names = vec!["one", "two", "three"];
    for name in names.iter() {
        println!("{}", name);
    }

    println!("{}", names[0]);
}

#[test]
fn test_into_iter() {
    // into_iter - 会消耗集合。
    // 在每次迭代中，集合中的数据本身会被提供。
    // 一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）了
    let names = vec!["one", "two", "three"];
    for name in names.into_iter() {
        println!("{}", name);
    }

    // Value used after being moved [E0382]
    // println!("{}", names.len());
}

#[test]
fn test_mut_iter() {
    // iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。
    let mut names = vec!["one", "two", "three"];
    for name in names.iter_mut() {
        *name = "four";
        println!("{}", name);
    }

    println!("{}", names[0]);
}
