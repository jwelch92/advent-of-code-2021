use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    solve_one(&contents);

}

fn risk_level(height: usize) -> usize {
    1 + height
}

// Find lowest points as coordinates and return that instead since finding sum is easy given the coords
// fn find_lowest_points(contents: &String)

// For part 2
// Using lowest points
// Recursively find all points nearby that are not 9's
// The size of the basin is the number of points
// Find the 3 biggest and multiply their sizes together

fn solve_one(contents: &String) -> usize {
    let (grid, rows , cols) = parse_input(contents);
    let lowest_points: Vec<(usize, usize)> = find_lowest_points(grid.to_vec(), rows, cols);
    let lowest_sum: usize = lowest_points.iter().map(|p| risk_level(grid[p.0][p.1])).sum::<usize>() as usize;
    println!("low {}", lowest_sum);
    lowest_sum
}

fn find_lowest_points(grid: Vec<Vec<usize>>, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut lowest_points: Vec<(usize, usize)> = vec![];
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
            lowest_points.push((y, x));
        }
        println!()
    }
    lowest_points
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
    assert_eq!(solve_two(&TEST_INPUT.to_string()), 1134)
}