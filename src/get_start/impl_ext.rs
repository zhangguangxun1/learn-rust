struct Person {
    name: String,
    age: u32,
}

impl Person {
    // 第一个参数不是 self 的方法称为关联函数, 类似其它语言的静态方法, 需要使用 Person::new 的方式调用
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    // &self 本身就是 self : &Self 的简写 实际上就是类似 a : &Self 的方式 Self 首字母大写是类型, 代表自己
    // 这样写的好处是后续可以通过 . 的方式调用这些方法
    fn get_age(&self) -> u32 {
        self.age
    }

    fn get_age_ext(a: &Self) -> u32 {
        a.age
    }
}

#[test]
fn test_read_struct() {
    let person = Person::new("John Smith".to_string(), 18);
    println!("{:?}", person.get_name());
    println!("{:?}", person.get_age());
}
