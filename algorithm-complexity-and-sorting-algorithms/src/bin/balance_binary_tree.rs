use std::fmt::Debug;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    height: i8, // Could be bool for Red and Black, but math is easier with a small int
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinData<T> {
    pub fn rot_left(mut self) -> Box<Self> {
        // result is the right node
        let mut res = match self.right.0.take() {
            Some(res) => res,
            None => return Box::new(self), //No right node how can we rotate?
        };

        // move left of right node to right of the start node
        self.right = BinTree(res.left.0.take());
        self.right.set_height();
        // set the results left node to the start node
        res.left = BinTree(Some(Box::new(self)));
        res.left.set_height();
        res.height = 1 + std::cmp::max(res.left.height(), res.right.height());

        res
    }

    pub fn rot_right(mut self) -> Box<Self> {
        // result is the left node
        let mut res = match self.left.0.take() {
            Some(res) => res,
            None => return Box::new(self), //No right node how can we rotate?
        };

        // move right of left node to left of the start node
        self.left = BinTree(res.right.0.take());
        self.left.set_height();
        // set the results right node to the start node
        res.right = BinTree(Some(Box::new(self)));
        res.right.set_height();
        // res.height = 1 + std::cmp::max(res.right.height(), res.left.height());
        res.height = 1 + std::cmp::max(res.left.height(), res.right.height());

        res
    }
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }

    pub fn height(&self) -> i8 {
        match self.0 {
            Some(ref t) => t.height,
            None => 0,
        }
    }

    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.height = 1 + std::cmp::max(t.left.height(), t.right.height());
        }
    }

    pub fn rot_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_left());
    }

    pub fn rot_right(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_right());
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        let rot_dir = match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                    if bd.left.height() - bd.right.height() > 1 {
                        1
                    } else {
                        0
                    }
                } else {
                    bd.right.add_sorted(data);
                    if bd.right.height() - bd.left.height() > 1 {
                        -1
                    } else {
                        0
                    }
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    height: 0,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }));
                0
            }
        };

        match rot_dir {
            1 => self.rot_right(),
            -1 => self.rot_left(),
            _ => self.set_height(),
        }
    }
}

impl<T: Debug> BinTree<T> {
    pub fn print_lfirst(&self, depth: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.print_lfirst(depth + 1);
            let mut spc = String::new();
            for _ in 0..depth {
                spc.push('.');
            }
            println!("{}↓{}({:?})", bd.height, spc, bd.data);
            bd.right.print_lfirst(depth + 1);
        }
    }
}

fn main() {
    // cargo run --bin balance_binary_tree

    let mut t = BinTree::new();
    t.add_sorted(4);
    t.add_sorted(5);
    t.add_sorted(6);
    t.add_sorted(10);
    t.add_sorted(1);
    t.add_sorted(94);
    t.add_sorted(54);
    t.add_sorted(3);

    for i in 0..100_000 {
        t.add_sorted(i);
    }
    t.print_lfirst(0);
}
