use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    solve_one(&contents);
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

#[cfg(test)]
mod test {
    use crate::solve_one;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_one() {
        assert_eq!(solve_one(&INPUT.to_string()), 37)
    }
}