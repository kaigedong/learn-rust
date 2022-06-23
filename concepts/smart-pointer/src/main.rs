mod qa;

/// 自动解引用，有如下特点：

fn main() {
    // 看Deref之前，看一下常规引用的解引用：
    normal_pointer();

    // Box 类型可以Deref
    deref_example_in_std();

    // 我们自己实现智能指针
    impl_smart_pointer();

    // 智能指针会在函数调用的时候，自动解引用
    deref_in_func_call();

    // QA
    // 为什么 i32 = &i32 + i32; works?
    qa::qa_ref_add();
    // deref 是否是move语义？
    qa::qa_deref_move1();
    // 另一个deref发生move的例子
    qa::qa_deref_move2();
}

// 通过*获取引用背后的值
fn normal_pointer() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

/// 对于智能指针（是结构体类型），如果直接进行*myStruct，编译器不知道怎么处理
/// 因此可以为智能指针结构体实现Deref特征
/// 实现了Deref的智能指针可以像普通引用一样，通过*进行解引用，例如Box<T>智能指针
/// 实际上Rust调用了如下方法： *(y.deref())
/// 首先调用 deref 方法返回值的常规引用，然后通过 * 对常规引用进行解引用，最终获取到目标值。
fn deref_example_in_std() {
    let x = Box::new(1);
    let _sum = *x + 1;
}

fn impl_smart_pointer() {
    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // NOTE: deref返回的是一个常规引用，可以被*进行解引用
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // 在实现 Deref 之前将会报错.
    let y = MyBox::new(5);
    assert_eq!(5, *y);

    {
        // 连续的隐式Deref转换
        fn display(s: &str) {
            println!("{}", s);
        }

        let s = MyBox::new(String::from("hello world"));
        display(&s);
    }

    // 另一个连续的隐式Deref转换
    // MyBox没有实现 to_string()方法,能调用 to_string，
    // 完全是因为编译器对 MyBox 应用了 Deref 的结果（方法调用会自动解引用）。

    let s = MyBox::new(String::from("hello, world"));
    // 对于 s1，我们通过两次 Deref 将 &str 类型的值赋给了它（赋值操作需要手动解引用）；
    let _s1: &str = &s;
    // 而对于 s2，我们在其上直接调用方法 to_string，
    let _s2: String = s.to_string();
}

// 函数和方法中，隐式Deref转换
// 说明：
/// 1. String 实现了 Deref 特征，可以在需要时自动被转换为 &str 类型
/// 2. &s 是一个 &String 类型，当它被传给 display 函数时，
/// 自动通过 Deref 转换成了 &str
/// 3. 必须使用 &s 的方式来触发 Deref(仅引用类型的实参才会触发自动解引用)
fn deref_in_func_call() {
    fn display(s: &str) {
        println!("{}", s);
    }

    let s = String::from("hello world");
    // &String -> &str
    display(&s)
}
