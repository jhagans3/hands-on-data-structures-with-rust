use std::cell::RefCell;
use std::rc::Rc;

type Rcc<T> = Rc<RefCell<T>>;

pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

#[derive(Debug)]
pub struct SkipNode<T: PartialOrd> {
    right: Option<Rcc<SkipNode<T>>>,
    down: Option<Rcc<SkipNode<T>>>,
    data: Rcc<T>,
}

impl<T: PartialOrd> SkipNode<T> {
    pub fn new(t: T) -> Self {
        SkipNode {
            right: None,
            down: None,
            data: rcc(t),
        }
    }

    // so far we never make an up node, so all we make is a linked list
    pub fn insert(&mut self, dt: T) -> Option<Rcc<SkipNode<T>>> {
        // bigger than right then gon right:
        if let Some(ref mut rt) = self.right {
            if dt > *rt.borrow().data.borrow() {
                return rt.borrow_mut().insert(dt);
            }
        }

        // has lower children try them
        if let Some(ref dw) = self.down {
            return match dw.borrow_mut().insert(dt) {
                Some(child) => match rand::random::<bool>() {
                    true => {
                        // dt is a pointer copy
                        let dt = child.borrow().data.clone();
                        let nn = SkipNode {
                            right: self.right.take(),
                            data: dt,
                            down: Some(child),
                        };

                        let res = rcc(nn);
                        self.right = Some(res.clone());
                        Some(res)
                    }
                    false => None,
                },
                None => None,
            };
        }

        // should be before right, at bottom node
        let mut nn = SkipNode::new(dt);
        nn.right = self.right.take();
        let res = rcc(nn);
        self.right = Some(res.clone());
        Some(res)
    }
}

fn main() {
    // cargo run --bin skip_list
    let mut s = SkipNode::new(4);
    s.insert(4);
    s.insert(6);
    s.insert(77);
    s.insert(84);
    s.insert(23);
    s.insert(1);

    println!("s: {:#?}", s);
}
