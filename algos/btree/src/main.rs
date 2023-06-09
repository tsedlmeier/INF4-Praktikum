use std::collections::hash_map::Keys;
use std::fs;
use std::env;

fn read_seq(v: &mut Vec<i8>, path: &String, cnt: &mut i32)
{
    let data = fs::read_to_string(path).expect("Unable to read file");
    for line in data.lines() {
        (*cnt)+=1;
        v.push( line.parse::<i8>().unwrap() );
    }
}

pub struct BTreeNode {
    m_: usize,
    keys_: Vec<i32>,
    refs_: Vec<Option<Box<BTreeNode>>>, // Option = None abbildung
    is_leaf_: bool
}
impl BTreeNode {
    fn new(m :usize) -> BTreeNode {
        Self { m_: m, keys_: Vec::with_capacity(2*m-1), refs_: Vec::with_capacity(2*m), is_leaf_: (true) }
    }
    fn insert(&mut self, key: i32) {
        if self.keys_.len() == self.m_ {
            return;
        }
        else {
            self.keys_.push(key);
        }
    }
    fn walk(self) {
        print!("( ");
        for k in self.keys_.iter() {
            print!("{0} ", k);
        }
        print!(" )");
    }
}

struct BTree {
    root_: Option<Box<BTreeNode>>
}
impl BTree {
    fn new(root: Option<Box<BTreeNode>>) -> BTree {
        BTree{ root_: root}
    }
    fn insert (&mut self, key: i32) {
        // let r : Option<Box<BTreeNode>> = self.root_;
        
    }
}

fn main() {
    let mut node : BTreeNode = BTreeNode::new(5);
    node.insert(1);
    node.insert(23);
    node.insert(1231);
    node.walk();
}
