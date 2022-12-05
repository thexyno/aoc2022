use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn from_to(s: &str) -> (usize, usize) {
    let mut spl = s.split("-");
    let first: usize = spl.next().unwrap().parse().unwrap();
    let second: usize = spl.next().unwrap().parse().unwrap();
    (first, second)
}

fn compare_tpl_contains(a1: usize, a2: usize, b1: usize, b2: usize) -> bool {
    (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2)
}
fn compare_tpl_overlaps(a1: usize, a2: usize, b1: usize, b2: usize) -> bool {
    (a1 <= b1 && a2 >= b1)
        || (b1 <= a2 && b2 >= a2)
        || (a1 <= b2 && a2 >= b1)
        || (b1 <= a1 && b2 >= a1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File open failed");
    let reader = BufReader::new(file);
    let mut num_ranges_contains = 0;
    let mut num_ranges_overlaps = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        let mut line = l.split(",");
        let (a1, a2) = from_to(line.next().unwrap());
        let (b1, b2) = from_to(line.next().unwrap());
        if compare_tpl_contains(a1, a2, b1, b2) {
            num_ranges_contains += 1;
        }
        if compare_tpl_overlaps(a1, a2, b1, b2) {
            num_ranges_overlaps += 1;
        }
    }
    println!("pairs contains: {}", num_ranges_contains);
    println!("pairs overlaps: {}", num_ranges_overlaps);
}
