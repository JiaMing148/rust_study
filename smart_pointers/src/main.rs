use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }

    fn set_parent(node: &Rc<Node>, parent: &Rc<Node>) {
        *node.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(Rc::clone(node));
    }
}

fn main() {
    let leaf = Rc::new(Node::new(3));
    let branch = Rc::new(Node::new(5));

    Node::set_parent(&leaf, &branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}