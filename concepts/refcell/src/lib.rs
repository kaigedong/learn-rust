// 一个例子，展示RefCell的用武之地
//
// 在下面的例子中，结构体中如果声明为 `messager: &'a T,`，
// 不允许后面的tests中再改变message的值了
// 本例子是RustBook中的<<RefCell与内部可变性>>的内容
//
// 一个能够通过的例子，在modify_example中，将引用的签名全部改为可变引用：`&mut`
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            Self {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        // mock_message 实现了send方法，因此，可以在limit_tracker里调用
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        // set_value是一个可变引用，没问题
        // set_value的时候，将会调用send方法
        limit_tracker.set_value(80);
        // NOTE: 因为RefCell是个智能指针（结构体），需要一些操作将值返回出来
        // 比如中间要检查是否符合借用规则，存储借用位置的信息等...
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
