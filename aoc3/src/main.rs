use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File open failed");
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line: Vec<char> = line.unwrap().chars().collect();
        let mut linechunk = line.chunks(line.len() / 2);
        let mut fst = linechunk.next().unwrap().iter();
        let snd = linechunk.next().unwrap();
        let eq = *fst.find(|x| snd.contains(x)).unwrap();
        sum += if eq >= 'a' {
            eq as u32 - 'a' as u32 + 1
        } else {
            eq as u32 - 'A' as u32 + 27
        };
    }
    println!("Part one: {}", sum);
    let file = File::open(&args[1]).expect("File open failed");
    let reader = BufReader::new(file);
    let linesvec: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    sum = 0;
    for lines in linesvec.chunks(3) {
        let fst = &mut lines[0].chars();
        let snd: Vec<char> = lines[1].chars().collect();
        let thi: Vec<char> = lines[2].chars().collect();
        let eq = fst.find(|x| snd.contains(x) && thi.contains(x)).unwrap();
        sum += if eq >= 'a' {
            eq as u32 - 'a' as u32 + 1
        } else {
            eq as u32 - 'A' as u32 + 27
        };
    }
    println!("Part two: {}", sum);
}
