use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    // println!("{}", contents);
    solve_one(&contents);
}

fn risk_level(height: u8) -> u8 {
    1 + height
}

fn solve_one(contents: &String) -> usize {
    let mut lowest: Vec<u8> = vec![];
    let (grid, rows , cols) = parse_input(contents);
    for y in 0..rows as isize {
        for x in 0..cols as isize {
            print!("{}", grid[y][x]);
            // check left
            if x -1 > 0 {
                // not lowest from left
                if grid[y][x -1] < grid[y][x] {
                    continue
                }
            }
            // check right
            if x + 1 < cols {
                // not lowest from left
                if grid[y][x + 1] < grid[y][x] {
                    continue
                }
            }
            // check up
            // check down

            lowest.push(grid[y][x]);
        }
        println!()
    }
    lowest.iter().map(|x| risk_level(*x)).sum::<u8>() as usize
}

fn solve_two(content: &String) -> usize {
    todo!();
    0
}

fn parse_input(contents: &String) -> (Vec<Vec<u8>>, usize, usize) {
    let lines: Vec<&str> = contents.lines().collect();
    let rows_size: usize = lines.len();
    let col_size: usize = lines[0].chars().count();
    println!("Build grid of size {} x {}", rows_size, col_size);


    let mut output = vec![vec![0; col_size]; rows_size];
    for (i, line) in contents.trim().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            println!("C is {:?}", c);
            output[i][j] = String::from(c).parse::<u8>().unwrap()
        }
    }
    (output, rows_size, col_size)
}


const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

#[cfg(test)]
#[test]
fn test_one() {
    assert_eq!(solve_one(&TEST_INPUT.to_string()), 15)
}

#[test]
fn test_two() {
    assert_eq!(solve_two(&TEST_INPUT.to_string()), 61229)
}