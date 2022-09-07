// 为什么`i32 = &i32 + i32` work?
// 因为std::ops::Add实现对该类型进行了实现
// https://stackoverflow.com/questions/57739755/how-could-rust-multiply-i32-with-i32
pub fn qa_ref_add() {
    let c = 1;
    let _d: i32 = &c + 1;
}

// 解引用操作是否需要转移所有权?
// https://rust-book.junmajinlong.com/ch6/05_re_understand_move.html

/// &vec![11, 22]中间产生了好几个临时变量，
/// 但最终有一个临时变量是vec的所有者，然后对这个变量进行引用，
/// 将引用赋值给变量v。
///
/// 使用*v解引用时，也产生了一个临时变量保存解引用得到的值，而这里就出现了问题。
/// 因为变量v只是vec的一个引用，而不是它的所有者，它无权转移值的所有权。
pub fn qa_deref_move1() {
    let _v = &vec![11, 22];
    // let vv = *_v; // error[E0507]: cannot move out of `*v` which is behind a shared reference
}

// NOTE: 注意，不要使用println!("{}", *a);或类似的宏来测试，
// 这些宏不是函数，它们真实的代码中使用的是&(*a)，因此不会发生所有权的转移。

// 其他的类似例子
pub fn qa_deref_move2() {
    let a = &"junmajinlong.com".to_string();
    // let b = *a;         // (1).取消注释将报错
    let _c = (*a).clone(); // (2).正确
                           // let _d = &*a; // (3).正确; clippy会警告

    let x = &3;
    let _y = *x; // (4).正确
}
