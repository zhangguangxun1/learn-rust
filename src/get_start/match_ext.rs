#[test]
fn test_ref() {
    // 对指针来说，解构（destructure）和解引用（dereference）要区分开，因为这两者的概念是不同的，和 C 那样的语言用法不一样。
    //
    // 解引用使用 *
    // 解构使用 &、ref、和 ref mut

    let ref_num = &10;
    match ref_num {
        &val => println!("val = {}", val),
    }
    match ref_num {
        val => println!("val = {}", val),
    }
    let num = 20;
    match num {
        ref val1 => println!("val = {}", val1),
        //val2 => println!("val = {}", val2),
    }

    let mut num = 3;
    match num {
        ref mut val1 => println!("val = {}", val1),
    }
}
