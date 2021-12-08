extern crate aoc;

fn main() {
    let contents: String = aoc::aoc::read_input("input.txt");
    solve_one(&contents);
    solve_two(&contents);
}

fn solve_one(contents: &String) -> usize {
    let mut numbers: Vec<isize> = contents.trim().split(",").map(|x| x.parse().unwrap()).collect();
    println!("{:?}", numbers);
    let middle = numbers.len() / 2;
    let (_, median, _): (&mut [isize], &mut isize, &mut [isize]) = numbers.select_nth_unstable(middle);
    println!("Median {}", median);
    let med = *median;
    let dist: isize = numbers.into_iter().map(|x| (med - x).abs()).sum();
    println!("Final fuel usage : {}", dist);
    dist as usize
}

fn solve_two(contents: &String) -> usize {
    let numbers: Vec<isize> = contents.trim().split(",").map(|x| x.parse().unwrap()).collect();
    // Get min and max
    let max: &isize = numbers.iter().max().unwrap();
    let min: &isize = numbers.iter().min().unwrap();
    println!("Numbers {:?} max={} min={}", numbers, max, min);
    // check each candidate between min and max for the potential "median"
    let least_fuel: isize = (*min..=*max)
        .into_iter()
        .map(|cand| {
            numbers.iter().map(|n| {
                // compute abs distance for each crab
                let dist = (cand - n).abs();
                // get the fuel cost for the distance where each additional space costs 1 more than the previous
                let a: isize = ((1 + dist) * dist) / 2;
                a
            }).sum()
        })
        .min()
        .unwrap();
    println!("Least fuel {}", least_fuel);
    least_fuel as usize
}

#[cfg(test)]
mod test {
    use crate::{solve_one, solve_two};

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_one() {
        assert_eq!(solve_one(&INPUT.to_string()), 37)
    }

    #[test]
    fn test_two() {
        assert_eq!(solve_two(&INPUT.to_string()), 168)
    }
}