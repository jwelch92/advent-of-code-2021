use std::fs;

const PART_ONE_DAYS: u32 = 80;

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");

    solve_one(&contents);
    solve_two(&contents);
}

fn solve_one(contents: &String) -> usize {
    let mut lanternfish: Vec<u8> = contents.lines().nth(0).unwrap().split(",").map(|x| x.parse::<u8>().unwrap()).collect();
    println!("{:?}", lanternfish);
    for _ in 0..PART_ONE_DAYS {
        let mut new_fish: u32 = 0;
        for idx in 0..lanternfish.len()  {
            match lanternfish[idx] {
                0 => {
                    lanternfish[idx] = 6;
                    new_fish += 1;
                },
                _ => {
                    lanternfish[idx] -= 1;
                }
            }
        }
        for _ in 0..new_fish {
            lanternfish.push(8);
        }
    }
    println!("Final score! {}", lanternfish.len());
    lanternfish.len()
}

fn solve_two(contents: &String) -> usize {
    let mut bucket = Bucket::new();
    let lanternfish: Vec<u8> = contents.lines().nth(0).unwrap().split(",").map(|x| x.parse::<u8>().unwrap()).collect();
    for fish in lanternfish {
        bucket.insert_fishy(fish as usize);
    }

    (0..256).for_each(|_| bucket.step());
    println!("Final score {}", bucket.score());

    bucket.score()
}

struct Bucket {
    fish: [usize; 9]
}

impl Bucket {
    fn new() -> Self {
        Bucket { fish: [0; 9] }
    }

    fn insert_fishy(&mut self, fish: usize) {
        self.fish[fish] += 1;
    }

    fn step(&mut self) {
        self.fish.rotate_left(1);
        self.fish[6] += self.fish[8];
    }

    fn score(&self) -> usize {
        self.fish.into_iter().sum()
    }
}

#[cfg(test)]
mod test {
    use crate::*;
     const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn one() {
        assert_eq!(solve_one(&INPUT.to_string()), 5934)
    }
    #[test]
    fn two() {
        assert_eq!(solve_two(&INPUT.to_string()), 26984457539);
    }
}