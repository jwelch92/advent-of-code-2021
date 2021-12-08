use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use itertools::Itertools;

fn main() {

    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");

    solve_one(&contents);
    solve_two(&contents);
    // parse_signal_patterns("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb");
}


fn solve_one(contents: &String) -> usize {
    println!("{}", contents);
    let mut count: usize = 0;
    for line in contents.trim().lines() {
        let (_, output) = line.split_once(" | ").unwrap();
        println!("{}", output);
        output.trim().split_whitespace().for_each(|x| {
            println!("Checking {}", x);
            match x.len() {
                2 | 4 | 3 | 7 => count += 1,
                _ => (),
            }
        })
    }
    println!("Found {} outputs for 1, 4, 7, 8", count);
    count
}

fn solve_two(content: &String) -> usize {
    let real: [&str; 10] = ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];
    let mut real_to_value: HashMap<&str, usize> = HashMap::new();
    real.iter().enumerate().for_each(|(i, s)| {
        real_to_value.insert(*s, i);
    });
    println!("{:?} {:?}", real, real_to_value);

    // TODO brute force using permutations???
    for i in "abcdefg".chars().permutations() {
        println!("{:?}", i);
    }



    0
}

//
// fn parse_signal_patterns(pat: &str) -> HashMap<&str, usize> {
//     let mut mapping: HashMap<&str, usize> = HashMap::new();
//     let mut reverse_mapping: HashMap<usize, HashSet<char>> = HashMap::new();
//
//     let mut inputs: Vec<&str> = pat.trim().split_whitespace().collect();
//     inputs.sort_by(|a, b | a.len().cmp(&b.len()));
//     println!("{:?}", inputs);
//
//     inputs.iter().for_each(|x| {
//         println!("x {}", x);
//         match x.len() {
//             2 => {
//                 mapping.insert(x, 1);
//                 reverse_mapping.insert(1, x.chars().collect());
//             },
//             4 => {
//                 mapping.insert(x, 4);
//                 reverse_mapping.insert(4, x.chars().collect());
//             },
//             3 => {
//                 mapping.insert(x, 7);
//                 reverse_mapping.insert(7, x.chars().collect());
//             },
//             7 => {
//                 mapping.insert(x, 8);
//                 reverse_mapping.insert(8, x.chars().collect());
//             },
//             _ => (),
//         };
//     });
//     // Okay so now we have 1, 4, 7, and 8
//     // From 1, 4, 7 we can determine the two right vertical segments
//     let right_seg_inter = reverse_mapping.get(&1).unwrap().intersection(reverse_mapping.get(&7).unwrap());
//     println!("{:?}", right_seg_inter);
//     println!("Mapping {:?} reverse {:?}", mapping, reverse_mapping);
//
//
//     mapping
// }

// 0 -> 6 seg
// 1 -> 2 seg
// 2 -> 5 seg
// 3 -> 5 seg
// 4 -> 4 seg
// 5 -> 5 seg
// 6 -> 6 seg
// 7 -> 3 seg
// 8 -> 7 seg
// 9 -> 6 seg


const TEST_INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

#[cfg(test)]
#[test]
fn test_one() {
    assert_eq!(solve_one(&TEST_INPUT.to_string()), 26)
}

#[test]
fn test_two() {
    assert_eq!(solve_two(&TEST_INPUT.to_string()), 61229)
}