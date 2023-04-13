// use std::env;
use std::fs;
use std::time::Instant;

struct Subset {
    sum   : i32,
    left  : usize,
    right : usize,
}

fn read_seq(v: &mut Vec<i8>, path: &String, cnt: &mut i32)
{
    let data = fs::read_to_string(path).expect("Unable to read file");
    for line in data.lines() {
        (*cnt)+=1;
        v.push( line.parse::<i8>().unwrap() );
    }
}

fn calc_partsum(v: &Vec<i8>, sub: &mut Subset)
{
    let n = v.len();
    let mut max : i32 = -128;

    for i in 1..n-1 {
        for j in i..n-1 {
            sub.sum = 0;
            for k in i..j {
                sub.sum += <i8 as Into<i32>>::into(v[k]);
            }
            if sub.sum > max {
                max = sub.sum;
                sub.left = i;
                sub.right = j;
            }
        }
    }
    sub.sum = max;
}

fn main() 
{
    let mut v: Vec<i8> = Vec::new();
    // let args: Vec<String> = env::args().collect();
    // let path = &args[1];
    let mut paths : Vec<String> = Vec::new();
    for i in 0..4 {
        let s = format!("../../../../input/seq{}.txt", i);
        paths.push(s);
    };



    for path in paths.iter() {
        let mut cnt : i32 = 0;
        let mut sub = Subset{ sum:0, left:0, right :0 }; 
        read_seq(&mut v, path, &mut cnt); 

        let now = Instant::now();
        calc_partsum(&v, &mut sub);
        let time = now.elapsed().as_micros() as f64;

        println!("--------------- N = {0} -----------------", cnt);
        println!(" max(Σ) = Z[{0}] + .. + Z[{1}] = {2} ", sub.left, sub.right, sub.sum);
        println!("Runtime: {0}us", time);
        // println!("Runtime: {0}s", time/1000000.0 );
    }   
}
