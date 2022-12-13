use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File open failed");
    let reader = BufReader::new(file);
    let mut treesizes: Vec<Vec<isize>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<isize> = line
            .chars()
            .map(|c| c.to_string().parse::<isize>().unwrap())
            .collect();
        treesizes.push(line);
    }
    let mut num_highest = 0;
    let mut checked: Vec<Vec<bool>> = vec![vec![false; treesizes[0].len()]; treesizes.len()];
    let tzl = treesizes.len() - 1;
    let mut cache: Vec<isize> = vec![-1; treesizes[0].len()];
    for (ind, line) in treesizes.iter().enumerate() {
        for (i, el) in line.iter().enumerate() {
            if cache[i] < *el && !checked[ind][i] {
                checked[ind][i] = true;
                num_highest += 1;
            }
            cache[i] = if *el > cache[i] { *el } else { cache[i] };
        }
    }
    let mut cache: Vec<isize> = vec![-1; treesizes[0].len()];
    for (ind, line) in treesizes.iter().rev().enumerate() {
        for (i, el) in line.iter().enumerate() {
            if cache[i] < *el && !checked[tzl - ind][i] {
                checked[tzl - ind][i] = true;
                num_highest += 1;
            }
            cache[i] = if *el > cache[i] { *el } else { cache[i] };
        }
    }
    let mut cache: Vec<isize> = vec![-1; treesizes.len()];
    for (x, _) in treesizes[0].iter().enumerate() {
        for (y, _) in treesizes.iter().enumerate() {
            if cache[y] < treesizes[y][x] && !checked[y][x] {
                checked[y][x] = true;
                num_highest += 1;
            }
            cache[y] = if treesizes[y][x] > cache[y] {
                treesizes[y][x]
            } else {
                cache[y]
            };
        }
    }
    let mut cache: Vec<isize> = vec![-1; treesizes.len()];
    let len0 = treesizes[0].len() - 1;
    for (x, _) in treesizes[0].iter().enumerate() {
        for (y, _) in treesizes.iter().enumerate() {
            if cache[y] < treesizes[y][len0 - x] && !checked[y][len0 - x] {
                checked[y][len0 - x] = true;
                num_highest += 1;
            }
            cache[y] = if treesizes[y][len0 - x] > cache[y] {
                treesizes[y][len0 - x]
            } else {
                cache[y]
            };
        }
    }
    println!("highest: {}", num_highest);
    let mut scenic_score_max = 0;
    for (y, line) in treesizes.iter().enumerate() {
        for (x, el) in line.iter().enumerate() {
            let left = {
                let mut to_return = 0;
                for n in (0..x).rev() {
                        to_return += 1;
                    if line[n] >= *el {
                        break;
                    }
                }
                to_return
            };
            let right = {
                let mut to_return = 0;
                for n in x + 1..line.len() {
                        to_return += 1;
                    if line[n] >= *el {
                        break;
                    }
                }
                to_return
            };
            let top = {
                let mut to_return = 0;
                for n in (0..y).rev() {
                        to_return += 1;
                    if treesizes[n][x] >= *el {
                        break;
                    }
                }
                to_return
            };
            let bottom = {
                let mut to_return = 0;
                for n in y + 1..treesizes.len() {
                        to_return += 1;
                    if treesizes[n][x] >= *el {
                        break;
                    }
                }
                to_return
            };
            let score = left * right * top * bottom;
            if score > scenic_score_max {
                scenic_score_max = score;
            }
        }
    }
    println!("score: {}", scenic_score_max);
}
