use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Graph {
    touched: HashSet<Vec2D>,
    knots: Vec<Vec2D>,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Vec2D {
    x: isize,
    y: isize,
}

impl Vec2D {
    fn x_dist(&self, other: Vec2D) -> isize {
        // println!("x_dist self: {}, other: {}, dist: {}",self.x,other.x,self.x-other.x);
        self.x - other.x
    }
    fn y_dist(&self, other: Vec2D) -> isize {
        // println!("y_dist self: {}, other: {}, dist: {}",self.y,other.y,self.y-other.y);
        self.y - other.y
    }
}

impl Graph {
    fn new(tailnum: usize) -> Self {
        let vec = Vec2D { x: 0, y: 0 };
        let mut elements: HashSet<Vec2D> = HashSet::new();
        elements.insert(vec);
        let tails = vec![vec; tailnum + 1];
        let g = Graph {
            touched: elements,
            knots: tails,
        };
        println!("{:?}", g);
        g
    }
    fn print(&self) {
        let mut to_return: String = "".to_owned();
        for y in -30..30 {
            for x in -50..50 {
                to_return = to_return + {
                    let a = self
                        .knots
                        .iter()
                        .enumerate()
                        .filter(|(_, el)| el.x == x && el.y == y)
                        .map(|(x, _)| x)
                        .last();
                    let mut a = match a {
                        Some(i) => i.to_string(),
                        None => ".".to_string(),
                    };
                    if self.touched.contains(&Vec2D { x, y }) {
                        a = "#".to_string();
                    };
                    a
                }
                .as_str();
            }
            to_return = to_return + "\n";
        }
        println!("{}", to_return);
    }
    fn mv(mut self, i: usize) -> Self {
        let xdist = self.knots[i-1].x_dist(self.knots[i]);
        let ydist = self.knots[i-1].y_dist(self.knots[i]);
        let mut x = self.knots[i].x;
        let mut y = self.knots[i].y;
        //println!("i: {}, x: {}, y: {}, xdist: {}, ydist: {}",i,x,y, xdist,ydist);
        if xdist > 1 {
            x += 1;
        } else if xdist < -1 {
            x -= 1;
        }

        if ydist > 1 {
            y += 1;
        } else if ydist < -1 {
            y -= 1;
        }

        if isize::abs(xdist) > 1 && isize::abs(ydist) == 1 {
            y = self.knots[i-1].y;
        }
        if isize::abs(xdist) == 1 && isize::abs(ydist) > 1 {
            x = self.knots[i-1].x;
        }
        self.knots[i].x = x;
        self.knots[i].y = y;
        // println!("i: {}, x: {}, y: {}, xdist: {}, ydist: {}",i,x,y, xdist,ydist);
        if self.knots.len() - 1 == i {
            self.touched.insert(self.knots[i]);
            self
        } else {
            self.mv(i + 1)
        }
    }
    fn left(mut self) -> Self {
        self.knots[0] = Vec2D {
            x: self.knots[0].x - 1,
            y: self.knots[0].y,
        };
        self.mv(1)
    }
    fn left_times(mut self, times: usize) -> Self {
        for _ in 0..times {
            self = self.left();
        }
        self
    }
    fn right(mut self) -> Self {
        self.knots[0] = Vec2D {
            x: self.knots[0].x + 1,
            y: self.knots[0].y,
        };
        self.mv(1)
    }
    fn right_times(mut self, times: usize) -> Self {
        for _ in 0..times {
            self = self.right();
        }
        self
    }
    fn up(mut self) -> Self {
        self.knots[0] = Vec2D {
            x: self.knots[0].x,
            y: self.knots[0].y - 1,
        };
        self.mv(1)
    }
    fn up_times(mut self, times: usize) -> Self {
        for _ in 0..times {
            self = self.up();
        }
        self
    }
    fn down(mut self) -> Self {
        self.knots[0] = Vec2D {
            x: self.knots[0].x,
            y: self.knots[0].y + 1,
        };
        self.mv(1)
    }
    fn down_times(mut self, times: usize) -> Self {
        for _ in 0..times {
            self = self.down();
        }
        self
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File open failed");
    let reader = BufReader::new(file);
    let mut graph = Graph::new(1);
    let mut graph2 = Graph::new(9);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split(" ");
        let dir = line.next().unwrap();
        let times: usize = line.next().unwrap().parse().unwrap();
        match dir {
            "L" => {
                graph = graph.left_times(times);
                graph2 = graph2.left_times(times)
            }
            "R" => {
                graph = graph.right_times(times);
                graph2 = graph2.right_times(times)
            }
            "U" => {
                graph = graph.up_times(times);
                graph2 = graph2.up_times(times)
            }
            "D" => {
                graph = graph.down_times(times);
                graph2 = graph2.down_times(times)
            }
            _ => panic!(),
        }
        //println!("{} {}", dir, times);
        //graph2.print();
    }
    println!("tailposses: {}", graph.touched.len());
    println!("tailposses long: {}", graph2.touched.len());
}
