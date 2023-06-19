use std::fs;
use std::env;
use regex::Regex;

pub struct Graph {
    nodes_: Vec<String>,
    edges_: Vec<(String,String)>
}

impl Graph {
    fn insert(&mut self, from: String, to: String) {
        if !self.nodes_.contains(&from) { 
            self.nodes_.push(from.clone());
        }
        if !self.nodes_.contains(&to) { 
            self.nodes_.push(to.clone());
        }
        let edge = (from, to);
        self.edges_.push(edge);
    }
    fn print_edges(&mut self) {
        for edges in self.edges_.iter() {
            println!("{0} -> {1}", edges.0, edges.1);
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
    let mut hoehle = Graph { nodes_: (Vec::with_capacity(1)), edges_: (Vec::with_capacity(1)) };
    inspect(&mut hoehle, path);
    hoehle.print_edges();
}
