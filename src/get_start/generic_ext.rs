use std::fmt::Debug;

#[test]
fn test_gen() {
    struct Gen<T>(T);

    let i_32 = Gen(2);
    println!("{}", i_32.0);

    #[derive(Debug)]
    struct A(i64);

    let i_gen_64 = Gen(A(2));
    let val = i_gen_64.0;
    println!("{:?}", val);
}

#[test]
fn test_generic() {
    struct Val {
        val: i32,
    }

    struct GenVal<T> {
        val: T,
    }

    impl Val {
        fn value(&self) -> i32 {
            self.val
        }
    }

    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.val
        }
    }

    let basic_val = Val { val: 1 };
    println!("{}", basic_val.value());

    let gen_val = GenVal { val: 1i32 };
    println!("{}", gen_val.value());
}

#[test]
fn test_generic2() {
    trait HasArea {
        fn area(&self) -> i64;
    }

    #[derive(Debug)]
    struct Rectangle {
        length: i64,
        width: i64,
    }
    impl HasArea for Rectangle {
        fn area(&self) -> i64 {
            self.length * self.width
        }
    }

    let rectangle = Rectangle {
        length: 10,
        width: 30,
    };

    print_debug(&rectangle);
    println!("{}", rectangle.area());
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}
