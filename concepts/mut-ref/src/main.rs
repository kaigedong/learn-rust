fn main() {
    // 拥有一个值和有一个可变引用的主要区别是：
    // 拥有者负责清理该值，当它不被需要时。
    // 除了这点，可以像拥有这个值一样对它做任何事情，但有一个说明：
    // 当你将一个可变引用后面的值移走，你必须在那个位置留下另一个值。
    // 如果不这样做，值的拥有者认为它需要清理这个值，但这时候已经没有需要的值了。

    println!("Hello, world!");

    let mut s = Box::new(42);
    replace_with_84(&mut s);
}

// 例子：你可以将一个可变引用后面的值移走
fn replace_with_84(s: &mut Box<i32>) {
    // 这不行，因为*s 将会是空
    // let was = *s;

    // 这样可以：
    let was = std::mem::take(s);
    // 这样也可以
    *s = was;

    // 我们可以交换在&mut后的值
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
}
