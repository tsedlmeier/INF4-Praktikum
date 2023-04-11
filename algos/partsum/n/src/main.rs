use std::env;
use std::fs;

struct Rets {
    max_   : i32,
    l_   : usize,
    r_   : usize,
}

fn calc_partsum_from_file(path: String) -> Rets
{
    let mut max = -128;
    let mut cur_sum : i32 = 0;
    let mut cur_left = 0;
    let mut left = 0;
    let mut right = 0;

    let data = fs::read_to_string(path).expect("Unable to read file");
    let mut i = 0;
    for line in data.lines() {
        cur_sum += line.parse::<i32>().unwrap();
        if cur_sum > max {
            max = cur_sum;
            left = cur_left;
            right = i;
        }
        if cur_sum < 0 {
            cur_sum = 0;
            cur_left = i+1;
        }
        i+=1;
    }

    return Rets{ max_:max, l_:left, r_:right };
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let rets = calc_partsum_from_file(path.to_string());

    println!("max partsum from Z[{0}..{1}]: {2} ", rets.l_, rets.r_, rets.max_);
}
