// use std::cell::RefCell;
// use std::rc::Rc;
//
// #[derive(Clone,Debug)]
// pub(crate) struct Node {
//     value: String,
//     next: Link,
//     prev: Link,
// }
//
// type Link = Option<Rc<RefCell<Node>>>;
//
// impl Node {
//     pub fn new(value: String) -> Node {
//             Node{value, next: None, prev: None }
//     }
//
//     pub fn append(&mut self, value: String) {
//         let new = Rc::new(RefCell::new(Node::new(value)));
//         match self.tail.take() {
//             Some(old) => {
//                 old.borrow_mut().next = Some(new.clone());
//                 new.borrow_mut().prev = Some(old);
//             }
//             None => self.head = Some(new.clone())
//         };
//     }
// }
//
