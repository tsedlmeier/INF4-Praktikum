use std::fs;
use std::env;
use std::vec;
use regex::Regex;
use std::collections::VecDeque;

#[derive(PartialEq, Copy, Clone)]
enum Color {
    WHITE,
    GRAY,
    BLACK
}
#[derive(PartialEq, Copy, Clone)]
enum Dist {
    DIST(usize),
    INF
}

impl std::ops::Add for Dist {
    type Output = Dist;

    fn add(self, other: Dist) -> Dist {
        match (self, other) {
            (Dist::DIST(x), Dist::DIST(y)) => Dist::DIST(x + y),
            (Dist::INF, _) | (_, Dist::INF) => Dist::INF,
        }
    }
}



pub struct Graph {
    nodes_: Vec<String>,
    edges_: Vec<(String,String)>,
    adj_list_: Vec<Vec<usize>>,
    n_: usize
}

impl Graph {
    fn insert(&mut self, from: String, to: String) {
        self.n_ += 1;
        if !self.nodes_.contains(&from) { 
            self.nodes_.push(from.clone());
        }
        if !self.nodes_.contains(&to) { 
            self.nodes_.push(to.clone());
        }
        let from_idx = self.nodes_.iter().position(|r| *r == from).unwrap();
        // println!("{} translates to {}", from, from_idx );
        let to_idx = self.nodes_.iter().position(|r| *r == to).unwrap();
        // println!("{} translates to {}", to, to_idx );
        let edge = (from, to);
        self.edges_.push(edge);
        // Ensure adj_list_ has enough capacity
        if self.adj_list_.len() < self.n_ {
            self.adj_list_.resize(self.n_, Vec::new());
        }
        self.adj_list_[from_idx].push(to_idx);
    }

    fn print_edges(&mut self) {
        for edges in self.edges_.iter() {
            println!("{0} -> {1}", edges.0, edges.1);
        }
    }

    fn print_idx(&mut self) {
        for i in 0..self.nodes_.len() {
            println!("{0} mapped to {1} ", self.nodes_[i], i);
        }
    }

    fn print_adj(&mut self) {
        for i in 0..self.nodes_.len() {
            println!("{} has neighbors", self.nodes_[i]);
            for neighbors in self.adj_list_[i].iter() {
                println!("{0}", neighbors);
            }
        }
    }
    // fn print_adjacency_list(&mut self) {
    //     for (node_index, neighbors) in self.adj_list_.iter().enumerate() {
    //         print!("Node {}: ", node_index);
    //         for &neighbor in neighbors {
    //             print!("{} ", neighbor);
    //         }
    //         println!();
    //     }
    // }

    fn bfs(&mut self, s: usize) {
        let mut colors: Vec<Color> = vec![Color::WHITE;self.nodes_.len()];
        let mut dist: Vec<Dist> = vec![Dist::INF;self.nodes_.len()];
        let mut pred: Vec<i32> = vec![-1; self.nodes_.len()];

        colors[s] = Color::GRAY;
        dist[s] = Dist::DIST(0);
        pred[s] = -1;
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(s);
        while !q.is_empty() {
            let u = q.pop_front();
            for v in &self.adj_list_[u.unwrap()] { // neighbors
                if colors[*v] == Color::WHITE {
                    colors[*v] = Color::GRAY;
                    dist[*v] = dist[*v] + Dist::DIST(1);
                    pred[*v] = u.unwrap() as i32;
                    q.push_back(*v);
                }
            }
            colors[u.unwrap()] = Color::BLACK;
        }
        // print shortest path
        for i in pred.iter() {
            print!("{} -> ", i);
        }
    }
}

fn inspect(graph: &mut Graph, path: &String)
{
    let delimiter = "\"";
    let re = Regex::new(delimiter).unwrap();

    let data = fs::read_to_string(path).expect("Unable to read file");
    for line in data.lines() {
        let result: Vec<&str> = re.split(line).collect();
        println!("{:?}", result);
        // assert!(result.get(1).is_none());
        match result.get(1) {
            Some(&" > ") => {graph.insert(result.get(0).unwrap().to_string(), result.get(2).unwrap().to_string());}
            Some(&" < ") => {graph.insert(result.get(2).unwrap().to_string(), result.get(0).unwrap().to_string());}
            Some(&" <> ") => {
                graph.insert(result.get(0).unwrap().to_string(), result.get(2).unwrap().to_string());
                graph.insert(result.get(2).unwrap().to_string(), result.get(0).unwrap().to_string());
            } 
            _ => ()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut hoehle = Graph { nodes_: (Vec::with_capacity(1)), edges_: (Vec::with_capacity(1)),
                             adj_list_:(Vec::with_capacity(1)), n_: (0)};
    inspect(&mut hoehle, path);
    println!("Index Table-----------------");
    hoehle.print_idx();
    println!("----------------------------");
    println!();
    hoehle.print_adj();
    hoehle.bfs(0);
}
