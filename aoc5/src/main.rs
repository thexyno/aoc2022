use regex::Regex;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug,Clone)]
struct Towers {
    towers: Vec<Vec<char>>,
}

impl Towers {
    fn new(s: &mut Vec<String>) -> Self {
        let re = Regex::new(r" (\d+)").unwrap();
        let tower_conf: Vec<usize> = re
            .captures_iter(&s.pop().unwrap()).into_iter()
            .map(|x| { x[1].parse::<usize>().unwrap()} )
            .collect();
        let mut towers: Vec<Vec<char>> = vec![Vec::new(); tower_conf.len()];
        for line in s.into_iter().rev() {
            for i in 0..tower_conf.len() {
                let el = line.chars().nth(1 + (i * 4)).unwrap();
                if el != ' ' {
                    towers[i].push(el);
                }
            }
        }
        Towers { towers }
    }
    fn move_crates(&mut self, num: usize, from: usize, to: usize) {
        let from_tower = from-1;
        let to_tower = to-1;
        let from_tower =  &mut self.towers[from_tower];
        let mut tmp: Vec<char> = vec![];
        (0..num).for_each(|_| {
            match from_tower.pop() {
                Some(x) => tmp.push(x),
                None => ()
            }
        });
        self.towers[to_tower].append(&mut tmp);
    }
    fn move_crates_inorder(&mut self, num: usize, from: usize, to: usize) {
        let from_tower = from-1;
        let to_tower = to-1;
        let from_tower =  &mut self.towers[from_tower];
        let mut tmp: Vec<char> = vec![];
        (0..num).for_each(|_| {
            match from_tower.pop() {
                Some(x) => tmp.push(x),
                None => ()
            }
        });
        tmp.reverse();
        self.towers[to_tower].append(&mut tmp);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File open failed");
    let reader = BufReader::new(file);
    let mut mode = 0;
    let mut to_parse: Vec<String> = vec![];
    let mut towers = Towers { towers: vec![]};
    let mut towers2 = Towers { towers: vec![]};
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in reader.lines() {
        let line = line.unwrap();
        match mode {
            0 => {
                if line.find("[").is_some() {
                    to_parse.push(line);
                } else {
                    to_parse.push(line);
                    towers = Towers::new(&mut to_parse);
                    towers2 = towers.clone();
                    mode += 1;
                }
            }
            1 => mode += 1,
            2 => {
                for l in re.captures_iter(&line) {
                    towers.move_crates(l[1].parse().unwrap(), l[2].parse().unwrap(), l[3].parse().unwrap());
                    towers2.move_crates_inorder(l[1].parse().unwrap(), l[2].parse().unwrap(), l[3].parse().unwrap());
                }

            }
            _ => (),
        }
    }
    print!("9000: ");
    for x in towers.towers.into_iter() {
        print!("{}", x.last().unwrap_or(&' '));
    }
    println!();
    print!("9001: ");
    for x in towers2.towers.into_iter() {
        print!("{}", x.last().unwrap_or(&' '));
    }
    println!();

}
