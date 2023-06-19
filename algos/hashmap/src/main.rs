use std::fs;
use std::env;

fn read_seq(v: &mut Vec<i8>, path: &String, cnt: &mut i32)
{
    let data = fs::read_to_string(path).expect("Unable to read file");
    for line in data.lines() {
        (*cnt)+=1;
        v.push( line.parse::<i8>().unwrap() );
    }
}

const A: f64 = 0.6180339887;

#[derive(PartialEq, Copy, Clone)]
enum MyOption<T> {
    Some(T),
    None,
    FreeAgain,
}

pub struct Hashmap {
    m_ : usize,
    a_ : Vec<MyOption<i32>>
}

fn hash1(k: i32, m: usize, i: usize ) -> usize { // m should be radix 2
    let k_f64 = k as f64;
    let m_f64 = m as f64;
    let i_f64 = i as f64;
    let h : f64 = (m_f64*(k_f64*A - (k_f64*A).floor())).floor();
    (h + i_f64 + 14.* i_f64*i_f64) as usize % m
}

fn hash2(k: i32, m: usize, i: usize ) -> usize { // m should be radix 2
    let k_f64 = k as f64;
    let m_f64 = m as f64;
    let h : usize = (m_f64*(k_f64*A - (k_f64*A).floor())).floor() as usize;
    if k % 2 == 0 {
        return (h + i^2)%m ;
    }
    else {
        return ((h - i^2) % m + m) % m;
    }
}

impl Hashmap {
    fn new(m: usize) -> Self {
        Hashmap { m_: (m), a_: (vec![MyOption::None; m]) }
    }

    fn insert(&mut self, key: i32) -> bool {
        let mut i: usize = 0;
        loop {
            let j = hash2(key, self.m_, i);
            println!("[INSERT] Calc Hash: {0} -> {1}", key,j);
            if self.a_[j] == MyOption::None {
                println!("[INSERT] Index {0} is empty -> insert {1}", j,key);
                self.a_[j] =  MyOption::Some(key);
                return true;
            }
            else {
                i+=1;
            }
            if i == self.m_ {break;}
        }
        return false;
    }
    fn search(&mut self, key: i32) -> usize {
        let mut i: usize = 0;
        loop {
            let j = hash2(key, self.m_, i);
            // println!("[SEARCH] Calc Hash: {0} -> {1}", key,j);
            if self.a_[j] == MyOption::Some(key) {
                // println!("[SEARCH] Found: {0} at Index {1}", key,j);
                return j;
            }
            i+=1;
            if self.a_[j] == MyOption::None || i == self.m_ {break;}
        }
        return 0;
    }

    fn alpha(&mut self) -> f64{
        let mut f :i32 = 0; //free
        let mut b :i32 = 0; //
        for entry in &self.a_ {
            match entry {
                MyOption::None => f += 1,
                MyOption::FreeAgain => f += 1,
                MyOption::Some(value) => b += 1,
            }
        }
        return b as f64 / self.m_ as f64
    }
    fn dump(&mut self) {
        let mut cnt = 0;
        for i in self.a_.iter() {
            match i {
                MyOption::None => println!("{ } | None ", cnt),
                MyOption::FreeAgain => println!("{ } | FreeAgain ", cnt),
                MyOption::Some(elem) => println!("{ } | { } ", cnt, elem)
            }
            cnt+=1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut z: Vec<i8> = Vec::new();
    let mut cnt : i32 = 0;
    read_seq(&mut z, path, &mut cnt);

    let mut map : Hashmap = Hashmap::new(31);

    for i in z.iter() {
        map.insert(*i as i32);
    }
    map.dump();
    println!("Serching {} --> IDX: {}", -87, map.search(-87));
}
