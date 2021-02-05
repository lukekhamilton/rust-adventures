use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    next: Link,
    prev: Link,
    value: String,
}

impl Node {
    fn new(value: String) -> Link {
        Some(Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        })))
    }
}

pub struct TransactinLog {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl TransactinLog {
    pub fn new_empty() -> TransactinLog {
        TransactinLog {
            head: None,
            tail: None,
            length: 0,
        }
    }
}
