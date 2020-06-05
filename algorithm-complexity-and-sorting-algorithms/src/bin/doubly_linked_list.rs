// Immutable outside, but can mutate the interior
use std::cell::RefCell;
// Reference counting pointer
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>,
}

#[derive(Debug)]
pub struct DbList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> DbList<T> {
    pub fn new() -> Self {
        DbList {
            first: None,
            last: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(r) => {
                // create new front object
                let new_front = Rc::new(RefCell::new(DbNode {
                    data,
                    next: Some(r.clone()),
                    prev: None,
                }));
                // tell the first object this is now in front of it
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                // put this on the front
                self.first = Some(new_front);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                // create new back object
                let new_back = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: Some(r.clone()),
                }));
                // tell the last object this is now behind it
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                self.last = Some(Rc::downgrade(&new_back));
                // put this on the back
                m.next = Some(new_back);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
}

fn main() {
    // cargo run --bin doubly_linked_list

    let mut dl = DbList::new();

    dl.push_front(6);
    dl.push_back(11);
    dl.push_front(5);
    dl.push_back(15);
    dl.push_front(4);

    println!("dl: {:#?}", dl);
}

// challenge - pop front and back
