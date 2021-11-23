#[cfg(test)]
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

use std::cell::RefCell;

use crate::Messenger;

#[test]
fn call_with_different_values() {
    let mut c = crate::Cacher::new(|a| {
        println!("cal: {}", a);
        a + 1
    });

    let v1 = c.value(2);
    let v1 = c.value(2);
    assert_eq!(v1, 3);

    let v2 = c.value(-2);
    assert_eq!(v2, -1);
}

#[test]
fn call_with_different_type() {
    let mut c = crate::Cacher::new(|a| {
        println!("\ncal: {}", a);
        7
    });

    let v = c.value("tetsss");
    let v = c.value("s");
    assert_eq!(v, 7);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

struct MockMessenger {
    // sent_messages: Vec<String>,
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            // sent_messages: vec![],
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
    let mut limit_tracker = crate::LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    // assert_eq!(mock_messenger.sent_messages.len(), 1);
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}
