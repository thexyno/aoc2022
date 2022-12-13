use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
enum OpVal {
    Old,
    Num(usize),
}


#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Op,
    op_val: OpVal,
    diff_by: usize,
    throw_true: usize,
    throw_false: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File open failed");
    let reader = BufReader::new(file);

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut lines = reader.lines();
    // parse input
    loop {
        match lines.next() {
            Some(_) => (),
            None => break,
        }
        let line = lines.next().unwrap().unwrap();
        let mut items = line.split(":");
        items.next();
        let items: Vec<usize> = items
            .next()
            .unwrap()
            .split(",")
            .map(|x| { x[1..].parse().unwrap()})
            .collect();
        let line = lines.next().unwrap().unwrap();
        let mut opgen = line.split("= ");
        opgen.next();
        let mut opgen = opgen.next().unwrap().split(" ");
        opgen.next();
        let opname = opgen.next().unwrap();
        let opval = opgen.next().unwrap();
        let op = match opname {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!(),
        };
        let op_val = match opval {
            "old" => OpVal::Old,
            x => OpVal::Num(x.parse::<usize>().unwrap()),
        };
        let diff_by = lines
            .next()
            .unwrap()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>().unwrap();
        let throw_true = lines
            .next()
            .unwrap()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>().unwrap();
        let throw_false = lines
            .next()
            .unwrap()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>().unwrap();
        monkeys.push(
            Monkey {
                items,
                op,
                op_val,
                diff_by,
                throw_true,
                throw_false,
            },
        );
        lines.next();
    }
    // rounds
    let mut cache: Vec<Vec<usize>> = vec![vec![];monkeys.len()];
    let mut inspected: Vec<usize> = vec![0;monkeys.len()];
    let test = monkeys.iter().map(|x| x.diff_by).fold(1, |x,y| x * y);
    for _r in 0..10000 {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            m.items.append(&mut cache[i]);
            for item in m.items.iter_mut() {
                inspected[i]+=1;
                let num = match m.op_val {
                    OpVal::Old => *item,
                    OpVal::Num(x) => x,
                };
                match m.op {
                    Op::Add => *item += num,
                    Op::Mul => {
                        *item *= num
                    },
                }
                //*item /= 3;
                *item = *item % test;
                if *item % m.diff_by == 0 {
                    cache[m.throw_true].push(*item);
                } else {
                    cache[m.throw_false].push(*item);
                }
            }
            m.items.clear();
        }
        println!("After round {}, the monkeys are holding items with these worry levels:",_r+1);
        for i in 0..monkeys.len() {
            println!("Monkey {}: {:?}",i,cache[i]);
        }
        println!();
    }
    for i in 0..inspected.len() {
        println!("Monkey {} inspected items {} times.",i,inspected[i]);
    }
    println!();
    inspected.sort();
    println!("Level of monkey business: {}",inspected[inspected.len()-2] * inspected[inspected.len() -1]);


}
