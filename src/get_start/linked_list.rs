enum LinkedList {
    // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
    Cons(i32, Box<LinkedList>),
    // Nil：末结点，表明链表结束
    Nil,
}

impl LinkedList {
    // 创建一个空的 List 实例
    fn new() -> LinkedList {
        LinkedList::Nil
    }

    // 处理一个 List，在其头部插入新元素，并返回该 List
    fn add_head(self, n: i32) -> LinkedList {
        LinkedList::Cons(n, Box::new(self))
    }

    // 返回 List 的长度
    fn len(&self) -> i32 {
        // 必须对 `self` 进行匹配（match），因为这个方法的行为取决于 `self` 的取值种类。
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的 `T`
        // 类型要好过匹配引用 `&T`。
        match *self {
            // 这个 len 方法是刚定义的 len, 递归的去求长度
            LinkedList::Cons(_, ref link) => 1 + link.len(),
            LinkedList::Nil => 0,
        }
    }

    // 返回列表的字符串表示（该字符串是堆分配的）
    fn stringify(&self) -> String {
        match *self {
            LinkedList::Cons(head, ref next) => format!("{}->{}", head, next.stringify()),
            LinkedList::Nil => "Nil".to_string(),
        }
    }
}

#[test]
fn test_linked_list() {
    let mut link = LinkedList::new();
    link = link.add_head(1);
    link = link.add_head(2);
    link = link.add_head(3);
    println!("link len: {}", link.len());
    println!("link: {}", link.stringify());
}
