use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File open failed");
    let reader = BufReader::new(file);
    let mut elf = 0;
    let mut cals: Vec<usize> = vec![0];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.as_str() == "" {
            elf += 1;
            cals.push(0);
        } else {
            cals[elf] += line.parse::<usize>().unwrap()
        }
    }
    cals.sort();
    println!("Max Calories: {}", cals[cals.len() -1]);
    println!("Max three Calories: {}", cals[cals.len()-3..].iter().sum::<usize>());
}
