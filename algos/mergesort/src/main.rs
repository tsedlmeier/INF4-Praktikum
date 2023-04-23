use std::fs;
use std::time::Instant;

fn read_seq(v: &mut Vec<i8>, path: &String, cnt: &mut i32)
{
    let data = fs::read_to_string(path).expect("Unable to read file");
    for line in data.lines() {
        (*cnt)+=1;
        v.push( line.parse::<i8>().unwrap() );
    }
}

fn mergesort(z: &Vec<i8>, l: usize, r: usize) -> Vec<i8>
{
    if l == r {
        return vec![ z[l] ];
    }

    let m:usize = (l+r)/2;
    let sorted_left = mergesort(&z, l, m);
    let sorted_right = mergesort(&z, m+1, r);

    let mut sorted: Vec<i8> = Vec::new();
    let mut i=0;
    let mut j=0;
    while  i<sorted_left.len() && j<sorted_right.len() {
        if sorted_left[i] < sorted_right[j]{
            sorted.push(sorted_left[i]);
            i+=1;
        }
        else {
            sorted.push(sorted_right[j]);
            j+=1;
        }
    }
    // Push back the rest
    while i<sorted_left.len() {
        sorted.push(sorted_left[i]);
        i+=1;
    }
    while j<sorted_right.len() {
        sorted.push(sorted_right[j]);
        j+=1;
    }

    return sorted;
}

fn printv(v: &Vec<i8>)
{
        for i in v.iter() {
            print!("{i},");
        }
}
fn main() 
{
    // let args: Vec<String> = env::args().collect();
    // let path = &args[1];
    let mut paths : Vec<String> = Vec::new();
    for i in 0..4 {
        let s = format!("../../../input/seq{}.txt", i);
        paths.push(s);
    };

    for path in paths.iter() {
        let mut z: Vec<i8> = Vec::new();
        let mut cnt : i32 = 0;
        // read_seq(&mut z, path.to_string()); 
        read_seq(&mut z, path, &mut cnt); 

    
    
        let now = Instant::now();
        let _sorted = mergesort(&z,0,z.len()-1);
        let time = now.elapsed().as_micros() as f64;

        println!("--------------- N = {0} -----------------", z.len());
        println!("Runtime: {0}us", time);
        // printv(&sorted);
    }  
}

