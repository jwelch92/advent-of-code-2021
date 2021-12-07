use std::cmp::{max, min};
use std::fs;

#[derive(Debug, Default)]
struct Line {
    a: Point,
    b: Point,
}

enum Direction {
    Vertical,
    Horizontal,
    Diagonal,
}

impl Line {
    // fn new() -> Self {
    //     Default::default()
    // }

    // fn is_vertical_or_horizontal(&self) -> bool {
    //     self.a.shares_axis(self.b)
    // }

    fn is_vertical(&self) -> bool {
        self.a.shares_x(self.b)
    }

    fn is_horizontal(&self) -> bool {
        self.a.shares_y(self.b)
    }

    fn direction(&self) -> Direction {
        if self.is_vertical() {
            Direction::Vertical
        } else if self.is_horizontal() {
            Direction::Horizontal
        } else {
            Direction::Diagonal
        }
    }
}


impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let mut l = s.split(" -> ");
        let a = l.next().unwrap();
        let b = l.next().unwrap();
        Line {
            a: Point::from(a),
            b: Point::from(b),
        }
    }
}


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    // fn shares_axis(&self, other: Point) -> bool {
    //     self.shares_x(other) || self.shares_y(other)
    // }

    fn shares_x(&self, other: Point) -> bool {
        self.x == other.x
    }

    fn shares_y(&self, other: Point) -> bool {
        self.y == other.y
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let mut l = s.split(",");
        let x = l.next().unwrap();
        let y = l.next().unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

fn print_grid(grid: &Vec<Vec<u16>>) {
    for row in grid {
        for col in row {
            match col {
                0 => print!("  "),
                _ => print!("{} ", col)
            }
            // print!("{} ", col)
        }
        println!()
    }
}

fn count_intersections(grid: &Vec<Vec<u16>>) -> usize {
    let mut count: usize = 0;
    for row in grid {
        for col in row {
            if *col > 1 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    let mut lines: Vec<Line> = vec![];
    let mut grid: Vec<Vec<u16>> = vec![vec![0; 1200]; 1200];
    for line in contents.lines() {
        lines.push(Line::from(line))
    }
    for line in &lines {
        match line.direction() {
            Direction::Diagonal => {
                let mut x_rate: isize = 1;
                let mut y_rate: isize = 1;

                if line.a.x > line.b.x {
                    x_rate = -1;
                }
                if line.a.y > line.b.y {
                    y_rate = -1;
                }
                let delta: isize = (line.b.x as isize - line.a.x as isize).abs();
                let x1 = line.a.x as isize;
                let y1 = line.a.y as isize;
                for i in 0..=delta {
                    let x = x1 + (i * x_rate);
                    let y = y1 + (i * y_rate);
                    grid[y as usize][x as usize] += 1;
                }
            }
            Direction::Horizontal => {
                let start = min(line.a.x, line.b.x);
                // Add one to account for inclusive points
                let end = max(line.a.x, line.b.x);
                println!("Start {} end {}", start, end);
                let y = line.a.y;
                for i in start..=end {
                    grid[y][i] += 1;
                }
            }
            Direction::Vertical => {
                let start = min(line.a.y, line.b.y);
                let end = max(line.a.y, line.b.y);
                println!("Start {} end {}", start, end);
                let x = line.a.x;
                for i in start..=end {
                    grid[i][x] += 1;
                }
            }
        }
    }
    print_grid(&grid);
    let intersections = count_intersections(&grid);
    println!("Got {} intersections", intersections);
}
