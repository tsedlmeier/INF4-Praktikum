use std::fs;
use std::time::Instant;

struct Rets {
    max_   : i32,
    l_   : usize,
    r_   : usize,
}

fn calc_partsum_from_file(path: &String, cnt: &mut i32) -> Rets
{
    let mut max = -128;
    let mut cur_sum : i32 = 0;
    let mut cur_left = 0;
    let mut left = 0;
    let mut right = 0;

    let data = fs::read_to_string(path).expect("Unable to read file");
    let mut i = 0;
    for line in data.lines() {
        (*cnt)+=1;
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
    // let args: Vec<String> = env::args().collect();
    // let path = &args[1];
    let mut paths : Vec<String> = Vec::new();
    for i in 0..4 {
        let s = format!("../../../../input/seq{}.txt", i);
        paths.push(s);
    };

    for path in paths.iter() {
        let mut cnt : i32 = 0; 
        let now = Instant::now();
        let rets = calc_partsum_from_file(path, &mut cnt);
        let time = now.elapsed().as_micros() as f64;

        println!("--------------- N = {0} -----------------", cnt);
        println!(" max(Σ) = Z[{0}] + .. + Z[{1}] = {2} ", rets.l_, rets.r_, rets.max_);
        // println!("Runtime: {0}s", time/1000000.0 );
        println!("Runtime: {0}us", time);
    }   
}
