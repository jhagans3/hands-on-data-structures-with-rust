use std::cell::RefCell;
use std::rc::Rc;

// internal mutability pattern
type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    value: String,
    next: SingleLink,
}

impl Node {
    // A nice and short way of creating a new node
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}
#[derive(Debug)]
struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);

        // Takes the value out of the option, leaving a None in its place.
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }
}

fn main() {
    // cargo run --bin transaction_log
    let mut db = TransactionLog::new_empty();

    db.append(String::from("a"));
    println!("after adding 'a', db: {:?}", db);
    db.append(String::from("b"));
    println!("after adding 'b', db: {:?}", db);
    db.append(String::from("c"));
    println!("after adding 'c', db: {:?}", db);

    db.pop();
    println!("after pop, db: {:?}", db);
}
