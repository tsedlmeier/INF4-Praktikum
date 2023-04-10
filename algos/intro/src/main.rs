use std::env;
use std::fs;

fn print_seq(v: &Vec<i8>)
{   
    for i in v.iter() 
    {
        println!("{i}"); // implicit \n
    }

}

// Function takes mutable reference (" &mut ")
fn read_seq(v: &mut Vec<i8>, path: String)
{
    // Read the file direct into string 
    let data = fs::read_to_string(path).expect("Unable to read file");
    // lines() detects "\n" 
    for line in data.lines() {
        v.push( line.parse::<i8>().unwrap() );
    }
}

fn main() 
{
    let mut v: Vec<i8> = Vec::new();

    // parsing cmd line 
    // args is owner
    let args: Vec<String> = env::args().collect();
    // path borrows args storage
    let path = &args[1];
    println!("recv Path: {}", path.to_string());

    // read_seq borrows v as mutable
    read_seq(&mut v, path.to_string()); 
    // print_seq borrows v as immutable (read only)
    print_seq(&v);
}
