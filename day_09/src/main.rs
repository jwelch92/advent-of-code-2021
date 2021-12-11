use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    // println!("{}", contents);
    println!("Part 1 {}", solve_one(&contents));
}

fn risk_level(height: usize) -> usize {
    1 + height
}

fn solve_one(contents: &String) -> usize {
    let mut lowest: Vec<usize> = vec![];
    let (grid, rows , cols) = parse_input(contents);
    for y in 0..rows {
        for x in 0..cols {
            print!("{}", grid[y as usize][x as usize]);
            // check left
            if x > 0 {
                // not lowest from left
                if grid[y][x -1] <= grid[y][x] {
                    continue
                }
            }
            // check right
            if x + 1 < cols {
                // not lowest from left
                if grid[y][x + 1] <= grid[y][x] {
                    continue
                }
            }

            // check up
            if y > 0 {
                if grid[y-1][x] <= grid[y][x] {
                    continue
                }
            }
            // check down
            if y < rows - 1  {
                if grid[y+1][x] <= grid[y][x] {
                    continue
                }
            }
            lowest.push(grid[y][x]);
        }
        println!()
    }
    println!("Low nums {:?}", lowest);
    let lowest_sum: usize = lowest.iter().map(|x| risk_level(*x)).sum::<usize>() as usize;
    println!("low {}", lowest_sum);

    lowest_sum

}

fn solve_two(content: &String) -> usize {
    todo!();
    0
}

fn parse_input(contents: &String) -> (Vec<Vec<usize>>, usize, usize) {
    let lines: Vec<&str> = contents.lines().collect();
    let rows_size: usize = lines.len();
    let col_size: usize = lines[0].chars().count();
    println!("Build grid of size {} x {}", rows_size, col_size);


    let mut output = vec![vec![0; col_size]; rows_size];
    for (i, line) in contents.trim().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            println!("C is {:?}", c);
            output[i][j] = String::from(c).parse::<usize>().unwrap()
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