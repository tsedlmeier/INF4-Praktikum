// use std::fs;
// use std::env;
use std::mem;

// #[derive(Clone)]
pub struct BTreeNode {
    m_: usize,
    n_: usize,
    keys_: Vec<i32>,
    refs_: Vec<BTreeNode>, // Option = None abbildung
    is_leaf_: bool
}

// Copyconstructor
impl Clone for BTreeNode {
    fn clone(&self) -> Self {
        Self {
            m_: self.m_,
            n_: self.n_,
            keys_: self.keys_.clone(),
            refs_: self.refs_.clone(),
            is_leaf_: self.is_leaf_,
        }
    }
}

impl BTreeNode {
    fn new(m :usize) -> Self {
        Self { m_: m, n_: 0, keys_: Vec::with_capacity(2*m-1), refs_: Vec::with_capacity(2*m), is_leaf_: (true) }
    }
    // For testing purpose
    // fn insert(&mut self, key: i32) {
    //     if self.keys_.len() == self.m_ {
    //         println!("Keys storage full");
    //         return;
    //     }
    //     else {
    //         self.keys_.push(key);
    //         self.n_ += 1;
    //     }
    // }

    fn print_keys(&self) {
        print!("( ");
        for k in self.keys_.iter() {
            print!("{0} ", k);
        }
        print!(" )");
    }

    fn walk(&self) {
        if self.is_leaf_ {
            self.print_keys();
            return ;
        }
        for i in 0..self.n_ {
            self.refs_[i].walk();
        }
    }

    fn b_tree_split_child(&mut self, i: usize) {
        let child = &mut self.refs_[i];
        let mut h = BTreeNode::new(self.m_);
        h.is_leaf_ = child.is_leaf_;
        h.n_ = self.m_-1;

        h.keys_.append(&mut child.keys_.split_off(self.m_));
        // for j in 0..2*m-1 {
        //     h.keys_[j] = child.keys_[j+m];
        // }
        if !child.is_leaf_ {
            h.refs_.append(&mut child.refs_.split_off(self.m_));
        }
        let key_that_moves_to_parent = child.keys_.remove(self.m_ - 1);
        self.keys_.insert(i, key_that_moves_to_parent);
        self.refs_.insert(i + 1, h);
    }

    fn b_tree_insert_in_node (&mut self, key: i32) {
        let mut i = self.n_;
        if self.is_leaf_ {
            while i >= 1 && key < self.keys_[i-1] {
                i -= 1;
            }
            self.keys_.insert(i, key);
            self.n_ += 1;
        }
        else {
            let mut j = 0;
            while j < self.n_ && key > self.keys_[j] {
                j += 1;
            }
            if self.refs_[j].n_ == 2*self.m_-1 {
                self.b_tree_split_child(j);
                if key > self.keys_[j] {
                    j += 1;
                }
            }
            self.refs_[j].b_tree_insert_in_node(key);
        }
    }
}

struct BTree {
    root_: BTreeNode
}

impl BTree {
    pub fn new(m: usize) -> Self {
        Self { root_: BTreeNode::new(m) }
    }

    fn insert (&mut self, key: i32) {
        let m = self.root_.m_;
        if self.root_.keys_.len() == 2 * self.root_.m_ - 1 {
            let h = mem::replace(&mut self.root_, BTreeNode::new(m));
            self.root_.refs_.push(h);
            self.root_.b_tree_split_child(0); // Call to split child node
        }
        self.root_.b_tree_insert_in_node(key);
    }

    fn walk(&self) {
        self.root_.walk();
    }
}

fn main() {
    let mut b_tree : BTree = BTree::new(5);
    b_tree.insert(1);
    b_tree.insert(2);
    b_tree.insert(3);
    b_tree.insert(3);
    b_tree.insert(3);
    b_tree.insert(3);
    b_tree.insert(3);
    b_tree.insert(3);
    b_tree.insert(3);
    b_tree.insert(3);
    b_tree.insert(3);
    b_tree.walk();
}
