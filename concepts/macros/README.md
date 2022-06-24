Rust 中宏分为两大类：声明式宏(declarative macros) macro_rules! 和 三种过程宏 (procedural macros):

- #[derive]，在之前多次见到过的派生宏，可以为目标结构体或枚举派生指定代码，例如 Debug trait

- 类属性宏(Attribute-like macro)，用于为目标添加自定义的属性

- 类函数宏(Function-like macro)，看上去就像是函数调用

Rust 语法包含数种"语法扩展"的形式。具体来说由以下四种：

1. `# [ $arg ]` 形式：比如 `#[derive(Clone)]`, `#[no_mangle]`,...
2. `#! [ $arg ]`形式：比如 `#![allow(dead_code)]`, `#![crate_name="blang]`, ...
3. `$name ! $arg`形式：比如 `println!("Hi!")`, `concat!("a", "b")`, ...
4. `$name ! $arg0 $arg1`形式：比如`macro_rule! dump { () => {}; }`

前两种形式被称为属性(attributes)，属性用来给条目(items)、表达式、语句加上注解。

属性有三类：

- 内置的属性(build-in attributes) ,由编译器实现
- 过程宏属性(proc-macro attributes)
- 派生属性(derive attributes)

## 声明宏

```sh
// 在一个宏中，可以有多个分支，宏根据不同的参数展开得到不同的代码。
// 每个分支可以接收多个参数，这些参数使用$符号开头，然后跟着一个token类型
//
// item 一个项（item），像一个函数，结构体，模块等
// block 一个块（block）（即一个语句块或一个表达式，由花括号所包围）
// stmt 一个语句(statement)
// pat 一个模式(pattern)
// expr 一个表达式(expression)
// ty 一个类型，type
// ident 一个标识符(identifier)
// path 一个路径（path），例如 foo, ::std::mem::replace, transmute::<_, int>, ...
// meta 一个元数据；位于#[...]和#![...]属性
// tt 一个词法树
// vis 一个可能为空的visibility限定词
```

例如：

```rust
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
```
