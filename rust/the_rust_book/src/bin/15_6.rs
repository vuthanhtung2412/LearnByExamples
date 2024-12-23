use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: RefCell<i32>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf1 = Rc::new(Node {
        value: RefCell::new(1),
        children: RefCell::new(vec![]),
    });

    *leaf1.value.borrow_mut() = 3;

    let branch = Rc::new(Node {
        value: RefCell::new(5),
        children: RefCell::new(vec![Rc::clone(&leaf1)]),
    });


    let mut l1    = Rc::clone(&branch.children.borrow_mut()[0]);
    *l1.borrow_mut().value.borrow_mut() = 2;

    println!("{:?}", leaf1);
}
