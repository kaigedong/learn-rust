// 参考Rust碎碎念 Rust宏：教程与示例（一）

// 声明宏就像是match表达式。
// macro_rules存在一些问题，[Rust计划在未来弃用它](https://github.com/rust-lang/rust/issues/39412)

// 简化版的vec!
#[macro_export]
macro_rules! vecm {
    ($($x:expr),*) => {
        {
            // let mut temp_vec = Vec::new();
            let mut temp_vec = vec![];
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/// 匹配add1(1); add1(1,1); add1(1,1,1)...
#[macro_export]
macro_rules! add1 {
    // second arm match add!(1), add!(2) etc
    ($a:expr) => {
        $a
    };
    // match like arm for macro
    // first arm match add!(1,2), add!(2,3) etc
    ($a:expr, $b:expr) => {
        // macro expand to this code
        $a + $b
    };
    // third arm match add!(1,2,...)
    ($($a:expr),*) => {
        // NOTE: 这个大括号一定不能少
        {
            let mut sum = 0;
            $(
                sum += $a;
            )*
            sum
        }
    }
}

#[macro_export]
macro_rules! add_as {
    ($a:expr, $b:expr, $typ:ty) => {
        $a as $typ + $b as $typ
    };
}

// Rust宏还支持可变数量的参数，非常类似于正则表达式。*被用于零个或更多的token类型，
// +被用于零个或者一个参数

// 重复的token被$()包裹，后面跟着一个分隔符和一个*或一个+表示这个token会重复的次数。
// 分隔符用于多个token之间相互区分。
// $()后面跟着*和+用于表示重复的代码块。
#[macro_export]
macro_rules! add3 {
    (
    // repeated block
    $($a:expr)
    //seperator
    ,
    // zero or more
    *
    ) => {
        {
            // to handle the case without any arguments
            0
            // block to be repeated
            $(+$a)*
        }
    };
}

// 上面的例子，一个额外的0使得语法有效。为了移除0，让add表达式像参数一样
// 我们需要创建一个新的宏：
// 以递归的方式分别处理每个token，每次处理单个token也更为简单。这个宏由三个分支：
// 第一个分支处理单个参数的情况
// 第二个分支处理两个参数的情况
// 第三个分支使用剩下的参数再次调用自身
#[macro_export]
macro_rules! add4 {
    ($a:expr) => {
        {
            $a
        }
    };
    ($a:expr, $b:expr) => {
        {
            $a+$b
        }
    };
    ($a:expr, $($b:tt)*) => {
        {
            $a+add4!($($b)*)
        }
    }
}

// 宏参数

#[cfg(test)]
mod test_my_vec {
    #[test]
    fn test_vec() {
        assert_eq!(vec![1, 2, 3, 4, 5], vecm![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_add() {
        assert_eq!(add1!(1, 2), 3);
        assert_eq!(add1!(1), 1);
        assert_eq!(add1!(1, 2, 3, 4, 5), 15);
    }

    #[test]
    fn test_add_as() {
        assert_eq!(add_as!(1i32, 2u8, i64), 3i64);
    }

    #[test]
    fn test_add3() {
        assert_eq!(add3!(1), 1);
        // Note compile!
        // assert_eq!(add3!(1,), 1);
        assert_eq!(add3!(1, 2), 3);
        assert_eq!(add3!(1, 2, 3), 6);
    }

    #[test]
    fn test_add4() {
        assert_eq!(add4!(1), 1);
        assert_eq!(add4!(1, 2, 3), 6);
    }
}
