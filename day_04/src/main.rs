use std::collections::{HashMap};
use std::fs;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Board {
    grid: [[Option<usize>; 5]; 5],
    index: HashMap<usize, Point>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    row: usize,
    col: usize,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            grid: [[Some(0); 5]; 5],
            index: Default::default(),
        }
    }
}

impl Board {
    fn new() -> Self {
        Default::default()
    }

    fn row_complete(&self, idx: usize) -> bool {
        return self.grid[idx].iter().all(|x| x.is_none());
    }

    fn score(&self) -> usize {
        self.index.keys().sum()
    }

    fn column_complete(&self, col: usize) -> bool {
        for i in 0..5 {
            if self.grid[i][col] != None {
                return false;
            }
        }
        return true;
    }

    fn add_number(&mut self, row: usize, col: usize, value: usize) {
        let p = Point {
            row,
            col,
        };
        self.index.insert(value, p);
        self.grid[row][col] = Some(value);
    }

    fn remove_number(&mut self, value: &usize) -> Option<Point> {
        match self.index.remove(&value) {
            None => None,
            Some(point) => {
                self.grid[point.row][point.col] = None;
                Some(point)
            }
        }
    }

    fn display(&self) {
        println!("{}", "----------------");
        for i in 0..4 {
            for j in 0..4 {
                match self.grid[i][j] {
                    Some(x) => print!("{} ", x),
                    None => print!("{} ", "X"),
                }
            }
            println!();
        }
        println!("{}", "----------------");
    }
}


fn build_boards(contents: &String) -> (Vec<usize>, Vec<Board>) {
    let mut random_numbers: Vec<usize> = vec![];
    let mut boards: Vec<Board> = vec![];
    let mut current_row = 0;
    for (no, line) in contents.lines().enumerate() {
        if no == 0 {
            // Get random nums
            random_numbers = line.split(",").map(|x| x.parse().unwrap()).collect();
            println!("{:?}", random_numbers);
        } else {
            // empty or board
            if line == "" {
                match boards.pop() {
                    Some(b) => {
                        boards.push(b);
                        current_row = 0;
                    }
                    None => (),
                }
                boards.push(Board::new());
            } else {
                match boards.pop() {
                    Some(mut b) => {
                        for (col, num) in line.split_whitespace().enumerate() {
                            b.add_number(current_row, col, num.parse().unwrap());
                        }
                        boards.push(b);
                        current_row += 1;
                    }
                    None => (),
                }
            }
        }
    }
    (random_numbers, boards)
}

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    let (random_numbers, mut boards) = build_boards(&contents);

    solve_one(&random_numbers, &mut boards);
    println!("{:?}", boards);
    solve_two(&random_numbers, &mut boards);
}

fn solve_two(random_numbers: &Vec<usize>, boards: &mut Vec<Board>) {
    let mut won_boards: Vec<Board> = vec![];
    let total_boards = boards.len();
    'outer: for called in random_numbers {
        for board in &mut *boards {
            if won_boards.contains(board) {
                continue
            }
            let point = board.remove_number(called);
            if point.is_some() {
                let p = point.unwrap();
                if board.row_complete(p.row) || board.column_complete(p.col) {
                    won_boards.push(board.to_owned());
                    board.display();
                }
                if won_boards.len() == total_boards {
                    // We're done!
                    println!("Final score {}", won_boards.last().unwrap().score() * called)
                }
            }
        }
    }
}


fn solve_one(random_numbers: &Vec<usize>, boards: &mut Vec<Board>) {
    'outer: for called in random_numbers {
        for board in &mut *boards {
            let point = board.remove_number(called);
            if point.is_some() {
                let p = point.unwrap();
                if board.row_complete(p.row) || board.column_complete(p.col) {
                    // We're done!
                    println!("We got it! Final score: {}", board.score() * called);
                    break 'outer;
                }
            }
        }
    }
}
