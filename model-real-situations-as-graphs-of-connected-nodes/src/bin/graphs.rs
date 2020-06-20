use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};

type Rcc<T> = Rc<RefCell<T>>;
pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

// edge list
pub struct EdgeListGraph<E, ID> {
    // Data on the edges at E
    // dont care about the nodes
    // cheap storage slow traversal
    v: Vec<(E, ID, ID)>,
}

// pointer based
// good for directed graphs as edges go one way,
// using weak pointers means the edge will fail safely
// if a node has been removed
// can stick edge data if need
pub struct RccGraph<T, E> {
    nodes: Vec<Rcc<RccNode<T, E>>>,
}

pub struct RccNode<T, E> {
    data: T,
    edges: Vec<(E, Weak<RefCell<RccNode<T, E>>>)>,
}

// map based
// map point from key to value normally quickly eg HashMap
pub struct MapGraph<T, E, ID: Hash> {
    mp: HashMap<ID, T>,
    edges: Vec<(E, ID, ID)>,
}

// Map pointer based
pub struct MapPGraph<T, E, ID: Hash + Eq> {
    data: HashMap<ID, (T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>,
}

fn main() {}
