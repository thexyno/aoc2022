use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File open failed");
    let reader = BufReader::new(file);
    let mut points = 0;
    let mut bpoints = 0;
    for line in reader.lines() {
        // first part
        let line = line.unwrap().as_str().as_bytes().to_owned();
        let abc = line[0] as char;
        let xyz = char::from_u32(line[2] as u32 - 23 as u32).unwrap();
        let a = abc as u32 - 'A' as u32;
        let b = xyz as u32 - 'A' as u32;
        points += 1 + b;
        if a == b {
            points += 3;
        } else if (a + 1) % 3 == b % 3 {
            points += 6;
        }
        // second part
        let c = match b {
            0 => (a + 2) % 3,
            1 => {
                bpoints += 3;
                a
            }
            2 => {
                bpoints += 6;
                (a + 1) % 3
            }
            _ => 0, // won't happen
        };
        bpoints += 1 + c;
    }
    println!("points: {}", points);
    println!("points (second part): {}", bpoints);
}
