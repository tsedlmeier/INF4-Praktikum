use std::fs;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::Write;

fn read_seq(v: &mut Vec<i8>, path: &String, cnt: &mut i32)
{
    let data = fs::read_to_string(path).expect("Unable to read file");
    for line in data.lines() {
        (*cnt)+=1;
        v.push( line.parse::<i8>().unwrap() );
    }
}

fn dump_runtimes(v: &Vec<i32>, dump_path: &String)
{
    let mut file = OpenOptions::new()
                                .create(true)
                                .write(true)
                                .open(dump_path.to_string())
                                .unwrap();
    let mut n = 1;
    for i in v.iter() {
        write!(file, "{n},").unwrap();
        write!(file, "{i}\n").unwrap();
        n = n*10;
    }
}
fn dump_results(v: &Vec<i8>, dump_path: &String)
{
    let mut file = OpenOptions::new()
                                .create(true)
                                .write(true)
                                .open(dump_path.to_string())
                                .unwrap();
    for i in v.iter() {
        write!(file, "{i}\n").unwrap();
    }
}

fn partition(z: &mut Vec<i8>, l: usize, r: usize) -> usize
{
    let pivot : i8 = z[r];
    let mut i : usize = l;
    let mut j : usize = r-1;
    while i < j {
        while z[i] < pivot {
            i=i+1;
        }
        while (j > l) && (z[j] >= pivot) {
            j=j-1;
        }
        if i < j {
            let tmp = z[i]; 
            z[i] = z[j];
            z[j] = tmp;
            i=i+1;
            j=j-1;
        }
    }
    if (i == j) && (z[i] < pivot) {
        i=i+1;
    }
    if z[i] != pivot {
        let tmp = z[i]; 
        z[i] = z[r];
        z[r] = tmp;
    }
    return i;
}

fn quicksort(z: &mut Vec<i8>, l: usize, r: usize)
{
    if l<r {
        let mut q = partition(z, l, r);
        if q==0 {q=1;} // prevent buffer overflow
        quicksort(z, l, q-1);
        quicksort(z, q+1, r);
    }

}

fn main()
{
    // let args: Vec<String> = env::args().collect();
    // let path = &args[1];

    let mut paths : Vec<String> = Vec::new();
    for i in 0..4 {
        let s = format!("../input/seq{}.txt", i);
        paths.push(s);
    };

    println!("\n------------------ quicksort -----------------");
    let mut runtimes : Vec<i32> = Vec::new();
    for path in paths.iter() {
        let mut z: Vec<i8> = Vec::new();
        let mut cnt : i32 = 0;
        let filename = &path.to_string()[9..13];

        read_seq(&mut z, path, &mut cnt);

        let now = Instant::now();

        let n = z.len()-1;
        quicksort(&mut z, 0, n);

        let time = now.elapsed().as_micros() as i32;
        runtimes.push(time);

        let results_path = &format!("./results/{}.dump", filename);
        dump_results(&z, results_path);

        println!("Sorting {0} - N = {1}", filename, cnt);
        println!("Writing sorted vector to {0}", results_path);
        println!("Runtime: {0}us", time);
        println!("----------------------------------------------");
    }
    let runtimes_path = "./results/runtimes.csv".to_string();
    dump_runtimes(&runtimes, &runtimes_path);
    println!("Writing Runtimes to {0}", runtimes_path);
}

