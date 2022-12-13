use std::{
    collections::VecDeque,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Ins {
    Noop,
    Addx(isize),
}
#[derive(Debug)]
enum Mode {
    Load,
    Execute,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File open failed");
    let reader = BufReader::new(file);
    let mut tape: VecDeque<Ins> = VecDeque::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split(" ");
        let op = line.next().unwrap();
        match op {
            "addx" => {
                let num: isize = line.next().unwrap().parse::<isize>().unwrap();
                tape.push_back(Ins::Addx(num));
            }
            "noop" => {
                tape.push_back(Ins::Noop);
            }
            _ => panic!(),
        }
    }
    let mut current_task_prog = 1;
    let mut clock = 0;
    let mut regx = 1;
    let mut output = 0;
    let mut mode: Mode = Mode::Load;
    loop {
        match mode {
            Mode::Load => {
                match tape.get(0) {
                    None => break,
                    Some(op) => match op {
                        Ins::Noop => {
                            //println!("load noop");
                            current_task_prog = 1;
                        }
                        Ins::Addx(_) => {
                            //println!("load addx {}",x);
                            current_task_prog = 2;
                        }
                    },
                }
                mode = Mode::Execute;
            }
            Mode::Execute => {
                // execute instruction
                current_task_prog -= 1;
                clock += 1;
                // do the output
                let crtcol = clock % 40;
                if crtcol - regx < 3 && crtcol - regx > -1 {
                    print!("#");
                } else {
                    print!(".");
                }
                if crtcol == 0 {
                    println!();
                }
                if clock == 20 || (clock - 20) % 40 == 0 {
                    output += clock * regx;
                }

                // after instruction
                // actually run the instruction
                if current_task_prog == 0 {
                    match tape.get(0) {
                        Some(op) => match op {
                            Ins::Noop => (),// println!("noop"),
                            Ins::Addx(x) => { regx += x },
                        },
                        None => break,
                    };
                    tape.pop_front();
                    mode = Mode::Load;
                }
            }
        }
    }
    println!("output: {}", output);
}
