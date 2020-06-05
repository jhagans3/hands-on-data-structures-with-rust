use std::cell::RefCell;
use std::rc::Rc;

// internal mutability pattern
type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

impl Node {
    // A nice and short way of creating a new node
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
            prev: None,
        }))
    }
}
#[derive(Debug, Clone)]
struct BetterTransactionLog {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl BetterTransactionLog {
    pub fn new_empty() -> BetterTransactionLog {
        BetterTransactionLog {
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

pub struct ListIterator {
    current: Link,
}

impl ListIterator {
    fn new(start_at: Link) -> ListIterator {
        ListIterator { current: start_at }
    }
}

impl Iterator for ListIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            }
            None => None,
        };
        result
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            }
            None => None,
        };
        result
    }
}

fn main() {
    let mut db = BetterTransactionLog::new_empty();

    db.append(String::from("a"));
    println!("after adding 'a', db: {:?}", db);
    db.append(String::from("b"));
    println!("after adding 'b', db: {:?}", db);
    db.append(String::from("c"));
    println!("after adding 'c', db: {:?}", db);

    db.pop();
    println!("after pop, db: {:?}", db);

    let mut it = ListIterator::new(db.clone().head);
    println!("it head next {:?}", it.next());
    println!("it head next back {:?}", it.next_back());

    let mut it = ListIterator::new(db.clone().tail);
    println!("it tail next back {:?}", it.next_back());
    println!("it tail next {:?}", it.next());
}
