use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    value: String,
    next: SingleLink
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None
        }))
    }
}

pub struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub leghth: i32
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog { head: None, tail: None, leghth: 0 }
    }

    pub fn append(&mut self, value: String) -> () {
        let new: Rc<RefCell<Node>> = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        }
        self.leghth += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            }else {
                self.tail.take();
            }
            self.leghth -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        }) 
    }
}

fn main() {
    let mut transaction: TransactionLog = TransactionLog::new_empty();

    transaction.append("Hello World".to_string());
    transaction.append("a1".to_string());

    let result1: Option<String> = transaction.pop();
    let result2: Option<String> = transaction.pop();

    match result2 {
        Some(s) => println!("{}", s),
        None => println!("Doesn't have any results!"),
    }
}