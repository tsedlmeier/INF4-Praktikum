use std::env;
use std::fs;

struct Subset {
    sum   : i32,
    left  : usize,
    right : usize,
}

fn read_seq(v: &mut Vec<i8>, path: String)
{
    let data = fs::read_to_string(path).expect("Unable to read file");
    for line in data.lines() {
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
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    read_seq(&mut v, path.to_string()); 

    let mut sub = Subset{
        sum : 0,
        left :0,
        right :0,
    }; 
    // let sum = calc_partsum(&v, &mut links, &mut rechts);
    calc_partsum(&v, &mut sub);

    println!("max partsum: {0}", sub.sum);
}
