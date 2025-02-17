use rand::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug)]
pub struct GraphErr {
    message: String,
}

impl GraphErr {
    pub fn new(s: &str) -> Self {
        GraphErr {
            message: s.to_string(),
        }
    }
}

pub trait Weighted {
    fn weight(&self) -> i32;
}

impl Weighted for i32 {
    fn weight(&self) -> i32 {
        *self
    }
}

#[derive(Debug)]
pub struct Route<ID> {
    pos: ID,
    path: Option<Rc<Route<ID>>>,
    len: i32,
}

impl<ID: Eq> Route<ID> {
    // wrap a new Route in an Rc
    pub fn start_rc(pos: ID) -> Rc<Self> {
        Rc::new(Route {
            pos,
            path: None,
            len: 0,
        })
    }

    // does the Rout contain a position
    pub fn contains(&self, id: &ID) -> bool {
        if self.pos == *id {
            return true;
        }
        match self.path {
            Some(ref p) => p.contains(id),
            None => false,
        }
    }
}

impl<ID: fmt::Debug> fmt::Display for Route<ID> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref p) = self.path {
            write!(f, "{}-{}-", p, self.len)?;
        }
        write!(f, "{:?}", self.pos)
    }
}

// Map pointer based
#[derive(Debug)]
pub struct Graph<T, E, ID: Hash + Eq> {
    data: HashMap<ID, (T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>,
}

impl<T, E, ID: Clone + Hash + Eq> Graph<T, E, ID> {
    pub fn new() -> Self {
        Graph {
            data: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: ID, data: T) {
        // node does not have edges yet
        self.data.insert(id, (data, Vec::new()));
    }

    pub fn add_edge(&mut self, ed_id: ID, from: ID, to: ID, edat: E) -> Result<(), GraphErr> {
        if !self.data.contains_key(&from) {
            return Err(GraphErr::new("'from' not in nodes"));
        }
        if let Some(ref mut dt) = self.data.get_mut(&to) {
            self.edges.insert(ed_id.clone(), (edat, from.clone(), to));
            dt.1.push(ed_id.clone());
        } else {
            return Err(GraphErr::new("'to' not in nodes"));
        }

        self.data.get_mut(&from).unwrap().1.push(ed_id);

        Ok(())
    }
}

impl<T, E: Weighted, ID: Clone + Hash + Eq> Graph<T, E, ID> {
    pub fn shortest_path(&self, from: ID, to: ID) -> Option<Rc<Route<ID>>> {
        self.shortest_path_route(Route::start_rc(from), to)
    }

    pub fn shortest_path_route(&self, from: Rc<Route<ID>>, to: ID) -> Option<Rc<Route<ID>>> {
        let mut to_set = HashSet::new();
        to_set.insert(to);

        self.closest(from, &to_set)
    }

    pub fn closest(&self, from: Rc<Route<ID>>, to: &HashSet<ID>) -> Option<Rc<Route<ID>>> {
        let mut visited = HashSet::new();
        let mut routes = Vec::new();
        routes.push(from);

        loop {
            let current_route = routes.pop()?;
            if to.contains(&current_route.pos) {
                return Some(current_route);
            }
            if visited.contains(&current_route.pos) {
                // no point in searching from the same place twice
                continue;
            }
            visited.insert(current_route.pos.clone());

            let exits = self.data.get(&current_route.pos)?;
            for eid in &exits.1 {
                let edge = self.edges.get(eid)?;
                let npos = if edge.1 == current_route.pos {
                    // opposite side of the edge to current pos
                    edge.2.clone()
                } else {
                    edge.1.clone()
                };
                let nlen = current_route.len + edge.0.weight();
                let nroute = Rc::new(Route {
                    pos: npos,
                    len: nlen,
                    // increase the RC count
                    path: Some(current_route.clone()),
                });
                if routes.len() == 0 {
                    routes.push(nroute);
                    continue;
                }
                // insert into the list sorted
                let mut iafter = routes.len() - 1;
                loop {
                    if routes[iafter].len > nlen {
                        // lowest element last
                        routes.insert(iafter + 1, nroute);
                        break;
                    }
                    if iafter == 0 {
                        // reached end
                        routes.insert(0, nroute);
                        break;
                    }
                    iafter -= 1;
                }
            }
        }
    }

    pub fn greedy_salesman(&self, start: ID) -> Option<Rc<Route<ID>>> {
        let mut to_visit: HashSet<ID> = self.data.keys().map(|k| k.clone()).collect();
        to_visit.remove(&start);
        let mut route = Route::start_rc(start.clone());
        while to_visit.len() > 0 {
            route = self.closest(route, &to_visit)?;
            to_visit.remove(&route.pos);
        }

        self.shortest_path_route(route, start)
    }

    pub fn complete_path(&self, path: &[ID]) -> Option<Rc<Route<ID>>> {
        if path.len() < 2 {
            return None;
        }
        let mut route = Route::start_rc(path[0].clone());
        for pos in &path[1..path.len() - 1] {
            if !route.contains(pos) {
                route = self.shortest_path_route(route, pos.clone())?;
            }
        }

        self.shortest_path_route(route, path[path.len() - 1].clone())
    }
}

impl<T, E: Weighted, ID: Clone + Hash + Eq + fmt::Debug> Graph<T, E, ID> {
    pub fn iter_salesman(&self, start: ID) -> Option<Rc<Route<ID>>> {
        let mut best_path: Vec<ID> = self.data.keys().map(|k| k.clone()).collect();
        best_path.shuffle(&mut rand::thread_rng());

        // move start to front
        for n in 0..best_path.len() {
            if best_path[n] == start {
                best_path.swap(0, n);
                break;
            }
        }
        //start and finish
        best_path.push(start);

        let mut best_route = self.complete_path(&best_path)?;
        let mut no_improvements = 0;
        loop {
            let mut path2 = best_path.clone();
            // not the ends
            let swap_a = (rand::random::<usize>() % (path2.len() - 2)) + 1;
            let swap_b = (rand::random::<usize>() % (path2.len() - 2)) + 1;
            path2.swap(swap_a, swap_b);
            let route2 = self.complete_path(&path2)?;
            if route2.len < best_route.len {
                println!("Improvement on {} = \n{}", best_route, route2);
                best_path = path2;
                best_route = route2;
                no_improvements = 0;
            }
            no_improvements += 1;
            if no_improvements >= 50 {
                return Some(best_route);
            }
        }
    }
}

fn main() -> Result<(), GraphErr> {
    // cargo run --bin graph
    let mut g = Graph::new();
    for x in vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'] {
        g.add_node(x, ());
    }

    g.add_edge('a', 'H', 'D', 6)?;
    g.add_edge('b', 'D', 'C', 18)?;
    g.add_edge('c', 'C', 'B', 10)?;
    g.add_edge('d', 'H', 'A', 7)?;
    g.add_edge('e', 'A', 'C', 4)?;
    g.add_edge('f', 'H', 'G', 5)?;
    g.add_edge('g', 'G', 'A', 8)?;
    g.add_edge('h', 'A', 'F', 3)?;
    g.add_edge('i', 'F', 'E', 15)?;
    g.add_edge('j', 'C', 'E', 12)?;

    println!("Hello, graph {:?}", g);
    println!("Shortes path A-D = {}", g.shortest_path('A', 'D').unwrap());
    println!("Shortes path H-B = {}", g.shortest_path('H', 'B').unwrap());

    println!("Greedy salesman A = {}", g.greedy_salesman('A').unwrap());
    println!("Iter salesman A = {}", g.iter_salesman('A').unwrap());

    Ok(())
}
