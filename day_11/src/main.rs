use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    solve_one(&contents);
    solve_two(&contents);
}

fn build_grid(contents: &String) -> Vec<Vec<usize>> {
    contents.trim().lines().map(|l| {
        l.trim().chars().map(|c| {
            String::from(c).parse().unwrap()
        }).collect()
    }).collect()
}

fn increment_grid(grid: &mut Vec<Vec<usize>>) {
    // let mut coords: Vec<(usize, usize)> = vec![];

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // let new_num = grid[y][x] + 1;
            grid[y][x] += 1;
            // if new_num > 9 {
            //     coords.push((y, x));
            // }
        }
    }
    // coords
}

fn set_flashed_to_zero(grid: &mut Vec<Vec<usize>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] > 9 {
                grid[y][x] = 0;
            }
        }
    }
}

fn print_grid(grid: &Vec<Vec<usize>>) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!()
    }
}

fn find_neighbors(pos: (usize, usize), grid: &mut Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = vec![];
    let (y, x) = pos;
    // hack, input is known size
    let rows = grid.len();
    let cols = grid[0].len();


    // check all positions starting from North and continuing clockwise
    // check north
    if y > 0 {
        let next_pos = (y - 1, x);
        neighbors.push(next_pos);
    }

    // check northeast
    if y > 0 && x < cols - 1 {
        let next_pos = (y - 1, x + 1);
        neighbors.push(next_pos);
    }

    // check east
    if x < cols - 1 {
        let next_pos = (y, x + 1);
        neighbors.push(next_pos);
    }
    // check southeast
    if x < cols - 1 && y < rows - 1 {
        let next_pos = (y + 1, x + 1);
        neighbors.push(next_pos);
    }

    // check south
    if y < rows - 1 {
        let next_pos = (y + 1, x);
        neighbors.push(next_pos);
    }

    // check southwest
    if y < rows - 1 && x > 0 {
        let next_pos = (y + 1, x - 1);
        neighbors.push(next_pos);
    }

    // check west
    if x > 0 {
        let next_pos = (y, x - 1);
        neighbors.push(next_pos);
    }

    // check northwest
    if y > 0 && x > 0 {
        let next_pos = (y - 1, x - 1);
        neighbors.push(next_pos);
    }
    neighbors
}

fn flash(grid: &mut Vec<Vec<usize>>, pos: (usize, usize), found: &mut Vec<(usize, usize)>) {
    let (y, x) = pos;
    if grid[y][x] > 9 && !found.contains(&pos) {
        found.push((y, x));
        for n in find_neighbors(pos, grid) {
            grid[n.0][n.1] += 1;
            flash(grid, n, found);
        }
    }
}

fn score_flashed(grid: &Vec<Vec<usize>>) -> usize {
    grid.iter()
        .map(|l| { l.iter().filter(|o| **o > 9).count() })
        .sum::<usize>()
}

fn solve_one(contents: &String) -> usize {
    let mut grid: Vec<Vec<usize>> = build_grid(contents);
    print_grid(&grid);
    let mut total_flashes: usize = 0;
    (0..100).for_each(|i| {
        // println!("Step {}", i);
        increment_grid(&mut grid);
        // increment grid
        // flash function
        // for each point greater than 9, find neighbors
        // halt is if point is > 9 and in the found vec
        let mut found: Vec<(usize, usize)> = vec![];
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] > 9 {
                    flash(&mut grid, (y, x), &mut found)
                }
            }
        }
        total_flashes += score_flashed(&grid);
        set_flashed_to_zero(&mut grid);
        // print_grid(&grid);
    });
    println!("Total flashes {}", total_flashes);
    total_flashes
}

fn solve_two(contents: &String) -> usize {
    let mut grid: Vec<Vec<usize>> = build_grid(contents);
    print_grid(&grid);
    let mut counter: usize = 0;
    loop {
        counter += 1;
        increment_grid(&mut grid);
        // increment grid
        // flash function
        // for each point greater than 9, find neighbors
        // halt is if point is > 9 and in the found vec
        let mut found: Vec<(usize, usize)> = vec![];
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] > 9 {
                    flash(&mut grid, (y, x), &mut found)
                }
            }
        }
        set_flashed_to_zero(&mut grid);
        if grid.iter()
            .all(|v| v.iter().all(|x| *x == 0)) {
            println!("It took {} steps to synchronize", counter);
            return counter;
        }
    }
}

const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_one() {
        assert_eq!(solve_one(&TEST_INPUT.to_string()), 1656)
    }

    #[test]
    fn test_two() {
        assert_eq!(solve_two(&TEST_INPUT.to_string()), 195)
    }
}


