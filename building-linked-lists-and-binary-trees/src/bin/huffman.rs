use std::collections::BTreeMap;

#[derive(Debug)]
pub enum HuffNode {
    Tree(Box<HuffNode>, Box<HuffNode>),
    Leaf(char),
}

impl HuffNode {
    pub fn print_lfirst(&self, depth: i32, dir: char) {
        match self {
            HuffNode::Tree(l, r) => {
                l.print_lfirst(depth + 1, '/');
                let mut spaces = String::new();
                for _ in 0..depth {
                    spaces.push('.');
                }
                println!("{}{}*", spaces, dir);
                r.print_lfirst(depth + 1, '\\');
            }
            HuffNode::Leaf(c) => {
                let mut spaces = String::new();
                for _ in 0..depth {
                    spaces.push('.');
                }
                println!("{}{}{}", spaces, dir, c);
            }
        }
    }

    pub fn encode_char(&self, c: char) -> Option<Vec<char>> {
        // could return vec of bool but chars print nicer
        // once you have this converting it to a byte
        // stream is pretty straight forward
        match self {
            HuffNode::Tree(l, r) => {
                if let Some(mut v) = l.encode_char(c) {
                    v.insert(0, '0');
                    return Some(v);
                }
                if let Some(mut v) = r.encode_char(c) {
                    v.insert(0, '1');
                    return Some(v);
                }
                None
            }
            HuffNode::Leaf(nc) => {
                if c == *nc {
                    Some(Vec::new())
                } else {
                    None
                }
            }
        }
    }

    pub fn encode_str(&self, s: &str) -> Option<Vec<char>> {
        let mut res = Vec::new();
        for c in s.chars() {
            let v = self.encode_char(c)?;
            res.extend(v.into_iter());
        }

        Some(res)
    }
}

pub struct HScore {
    node: HuffNode,
    score: i32,
}

pub fn build_tree(s: &str) -> HuffNode {
    let mut map = BTreeMap::new();
    for c in s.chars() {
        // if map has already added 1 else put 1
        let number_of_occurrence = *map.get(&c).unwrap_or(&0);
        map.insert(c, number_of_occurrence + 1);
    }

    let mut tlist: Vec<HScore> = map
        .into_iter()
        .map(|(key, score)| HScore {
            node: HuffNode::Leaf(key),
            score,
        })
        .collect();

    while tlist.len() > 1 {
        let last = tlist.len() - 1;
        for i in 0..last - 1 {
            if tlist[i].score < tlist[last - 1].score {
                tlist.swap(i, last - 1);
            }
            if tlist[last - 1].score < tlist[last].score {
                tlist.swap(last - 1, last);
            }
        }
        let a_node = tlist.pop().unwrap(); // len >=2
        let b_node = tlist.pop().unwrap(); // len >=2
        let new_node = HuffNode::Tree(Box::new(a_node.node), Box::new(b_node.node));
        tlist.push(HScore {
            node: new_node,
            score: a_node.score + b_node.score,
        });
    }

    tlist.pop().unwrap().node
}

fn main() {
    let s = "at an apple app";
    println!("{}", s);
    let t = build_tree(s);
    t.print_lfirst(0, '<');

    println!("n = {:?}", t.encode_char('n'));
    println!("encoded string: {:?}", t.encode_str(s));
}
// challenge
// Write the decoder you will want to
// convert the input char vec into an iterator
