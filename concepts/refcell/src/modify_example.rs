// 一个例子，展示RefCell的用武之地
//
// 在下面的例子中，结构体中如果声明为 `messager: &'a T,`，
// 不允许后面的tests中再改变message的值了
// 本例子是RustBook中的<<RefCell与内部可变性>>的内容
pub trait Messenger {
    fn send(&mut self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a mut T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &mut T, max: usize) -> LimitTracker<T> {
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

// 在一个结构体中，引用了一个数据，这个数据类型是不可变的。
// 这个结构体调用了某种方法，想要改变引用的这个数据
// 这时候，可以按照下面的方式，将这个数据从声明的时候，就声明为可变的；
// 然后在结构体中的引用，也声明为可变引用，实现的方法也是可变引用；
// 这样就能由结构体改变这个引用的数据。
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&mut self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mut mock_messenger = MockMessenger::new();
        // mock_message 实现了send方法，因此，可以在limit_tracker里调用
        let mut limit_tracker = LimitTracker::new(&mut mock_messenger, 100);

        // set_value是一个可变引用，没问题
        // set_value的时候，将会调用send方法
        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
