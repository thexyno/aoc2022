use std::{
    collections::{HashSet, VecDeque},
    env,
    fs::File, io::{BufReader, BufRead},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File open failed");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.chars();
        let mut ind = 0;
        let mut chr1: VecDeque<char> = VecDeque::new();
        let mut chr2: VecDeque<char> = VecDeque::new();
        let mut fst_solved = false;
        for c in line {
            let iter1 = chr1.iter();
            let iter2 = chr2.iter();
            let set1: HashSet<char> = iter1.map(|x| *x).collect();
            let set2: HashSet<char> = iter2.map(|x| *x).collect();
            //println!("ind: {}, set1: {:?}, set2: {:?}", ind,set1,set2);
            if set1.len() == 4 && !fst_solved {
                println!("1st: {}, {:?}", ind, set1);
                fst_solved = true;
            }
            if set2.len() == 14 {
                println!("2nd: {}, {:?}", ind, set2);
                break;
            }
            if chr1.len() == 4 {
            chr1.pop_front();
            }
            if chr2.len() == 14 {
            chr2.pop_front();
            }
            chr1.push_back(c);
            chr2.push_back(c);
            ind += 1;
        }
    }
}
