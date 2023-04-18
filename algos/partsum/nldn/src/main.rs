use std::fs;
use std::time::Instant;

struct Rets {
    max_   : i32,
    l_   : usize,
    r_   : usize,
}

fn read_seq(v: &mut Vec<i8>, path: &String, cnt: &mut i32)
{
    let data = fs::read_to_string(path).expect("Unable to read file");
    for line in data.lines() {
        (*cnt)+=1;
        v.push( line.parse::<i8>().unwrap() );
    }
}

fn find_between(z:&Vec<i8>, l:usize, m:usize, r:usize) -> Rets
{
    let mut left = 0;
    let mut right = 0;
    let mut left_max = -128;
    let mut right_max = -128;
    let mut sum = 0;

    for i in (l..m+1).rev() { // careful with bound
        sum += <i8 as Into<i32>>::into(z[i]);
        if sum > left_max {
            left_max = sum;
            left = i;
        }
    }
    sum = 0;
    for i in m+1..r {
        sum += <i8 as Into<i32>>::into(z[i]);
        if sum > right_max {
            right_max = sum;
            right = i;
        }
    }
    return Rets { max_:left_max+right_max, l_:left, r_:right };
}

fn calc_partsum(z:&Vec<i8>, l:usize, r:usize) -> Rets
{
    if l == r {
        return Rets{ max_:<i8 as Into<i32>>::into(z[l]), l_:l, r_:r };
    }

    let m:usize = (l+r)/2;
    let left = calc_partsum(&z,l,m);        // case 1
    let right = calc_partsum(&z,m+1,r);     // case 2
    let middle = find_between(&z,l,m,r);    // case 3

    // left sum was largest
    if left.max_ >= right.max_ && left.max_ >= middle.max_ {
        return left;
    }
    // right sum was largest
    if right.max_ >= left.max_ && right.max_ >= middle.max_ {
        return right;
    }
    // middle sum was largest
    return middle;

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
        let mut z: Vec<i8> = Vec::new();
        let mut cnt : i32 = 0;
        // read_seq(&mut z, path.to_string()); 
        read_seq(&mut z, path, &mut cnt); 

        let now = Instant::now();
        let rets = calc_partsum(&z,0,z.len()-1);
        let time = now.elapsed().as_micros() as f64;

        println!("--------------- N = {0} -----------------", cnt);
        println!(" max(Σ) = Z[{0}] + .. + Z[{1}] = {2} ", rets.l_, rets.r_, rets.max_);
        // println!("Runtime: {0}s", time/1000000.0 );
        println!("Runtime: {0}us", time);
    }   
}

