use std::fs;
use std::env;
// use std::thread::current;
// use std::time::Instant;

fn read_seq(v: &mut Vec<i8>, path: &String, cnt: &mut i32)
{
    let data = fs::read_to_string(path).expect("Unable to read file");
    for line in data.lines() {
        (*cnt)+=1;
        v.push( line.parse::<i8>().unwrap() );
    }
}

pub struct BinTreeElement {
    key_: i32,
    left_: Option<Box<BinTreeElement>>, // Option = None abbildung
    right_: Option<Box<BinTreeElement>> // Box --> smart ptr
}

pub trait Tree { // trait = interface
    fn new() -> Self;
    fn traverse(&self, node: &Option<Box<BinTreeElement>>, f: &dyn Fn(i32,usize), level: usize);
    fn search(&self, node: &Option<Box<BinTreeElement>>, key: i32) -> bool;
    fn insert(&mut self, key: i32);
    fn successor(&self, key: i32) -> Option<i32>;
    fn walklevel(&self, f: &dyn Fn(Vec<i32>, usize));
}

pub struct BinSearchTree {
    root: Option<Box<BinTreeElement>>
}

impl Tree for BinSearchTree{

     fn new () -> BinSearchTree {
       return BinSearchTree {root: None};
     }

    fn traverse(&self, node: &Option<Box<BinTreeElement>>, f: &dyn Fn(i32,usize), level: usize){
        if node.is_none() {
            return;
        }
        let current_node = node.as_ref().unwrap();
        if self.root.is_some() {
            self.traverse(&current_node.left_, f, level+1);
            f(current_node.key_, level);
            self.traverse(&current_node.right_, f, level+1);
        }
    }


    fn search(&self, node: &Option<Box<BinTreeElement>>, key: i32) -> bool
    { 
        if node.is_none() {
            return false;
        }

        let current_node = node.as_ref().unwrap();
        if self.root.is_none() || key == current_node.key_ {
            return true; 
        }
        if key < current_node.key_ {
            return self.search(&current_node.left_, key)
        }
        else {
            return self.search(&current_node.right_, key)
        }
    }

    fn insert(&mut self, key: i32){
        let new_node = Box::new(BinTreeElement {key_: key, left_: None, right_: None });

        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }

        let mut current = self.root.as_mut().unwrap();
        loop{
            if key < current.key_ {
                if current.left_.is_none() {
                    current.left_ = Some(new_node);
                    break;
                }
                current = current.left_.as_mut().unwrap();
            } else if key > current.key_ {
                if current.right_.is_none() {
                    current.right_ = Some(new_node);
                    break;
                }
                current = current.right_.as_mut().unwrap();
            } else {
                // Key already exists, handle as desired (e.g., update value or ignore)
                break;
            }
        }
    }

    fn successor(&self, key: i32) -> Option<i32> {
        let mut current_node = &self.root;
        let mut successor: Option<i32> = None;

        while let Some(node) = current_node {
            if node.key_ > key {
                successor = Some(node.key_);
                current_node = &node.left_;
            } else {
                current_node = &node.right_;
            }
        }
        return successor;
    }

    fn walklevel(&self, f: &dyn Fn(Vec<i32>, usize)) {
        let start_node = &self.root;
        walk_level_order(start_node, f);    // Starte die Niveau-Traversierung des Binärbaums
    }
}

fn walk_level_order(start: &Option<Box<BinTreeElement>>, f: &dyn Fn(Vec<i32>, usize)) {
    if let Some(node) = start {
        let mut queue: Vec<(&Box<BinTreeElement>, usize)> = Vec::new();
        queue.push((node, 0));

        while !queue.is_empty() {
            let (current_node, level) = queue.remove(0);

            let mut values: Vec<i32> = Vec::new();

            values.push(current_node.key_);

            if let Some(left_child) = &current_node.left_ {
                queue.push((left_child, level + 1));
            }

            if let Some(right_child) = &current_node.right_ {
                queue.push((right_child, level + 1));
            }

            f(values, level);
        }
    }
}

fn print_level(values: Vec<i32>, level: usize) {
    print!("Level {}: ", level);
    for value in values {
        print!("{} ", value);
    }
    println!();
}

fn print(val: i32, level: usize)
{
    println!("{}{}", "|##".repeat(level), val);
    // println!("{0}\n", val)
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut z: Vec<i8> = Vec::new();
    let mut cnt : i32 = 0;
    read_seq(&mut z, path, &mut cnt);

    let mut binary_tree = BinSearchTree::new();

    for i in z.iter() {
        binary_tree.insert(*i as i32);
    }

    let flag = binary_tree.search(&binary_tree.root, 42);
    println!("Search for 42 : Found?{0} ", flag);

    binary_tree.traverse(&binary_tree.root, &print, 0);

    let suc = binary_tree.successor(52);
    println!("Successor for 52 : {0} ", suc.unwrap());

    binary_tree.walklevel(&print_level);


}

