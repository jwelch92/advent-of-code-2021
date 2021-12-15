use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    // use hashmap to track pairs->count
    // updates happen simultaneously so make sure to copy the pairs on each step to avoid dupe counting or inserting too many
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    solve_one(&contents);
    solve_two(&contents);
}


fn run(steps: usize, contents: &String) -> usize {
    let polymer_template = contents.lines().nth(0).unwrap().trim().to_string();
    println!("Polymer {}", polymer_template);
    let insertion_rule: HashMap<String, String> = contents.lines().filter(|l| {
        !l.trim().is_empty() && l.contains("->")
    }).map(|l| {
        let (k, v) = l.split_once(" -> ").unwrap();
        (k.to_string(), v.to_string())
    }).collect();
    println!("{:?}", insertion_rule);
    let mut polymer: HashMap<String, usize> = HashMap::new();
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    let pchars: Vec<char> = polymer_template.chars().collect();
    // Base count of each char
    for i in 0..pchars.len() {
        *char_counts.entry(pchars[i]).or_insert(0) += 1;
    }
    // Base pair count
    for i in 0..pchars.len()-1 {
        *polymer.entry(String::from_iter([pchars[i], pchars[i+1]])).or_insert(0) += 1;
    }
    // println!("Polymer before {:?}", polymer);
    // println!("Counter before: {:?}", char_counts);

    for st in 0..steps {
        do_insertion(&mut polymer, &mut char_counts, &insertion_rule);
        // println!("Polymer step {} {:?}", st, polymer);
        // println!("Count step {} {:?}", st, char_counts);
    }
    println!("Polymer {:?}", polymer);
    println!("Counter: {:?}", char_counts);
    let max = *char_counts.values().max().unwrap();
    let min = *char_counts.values().min().unwrap();
    max - min
}

fn do_insertion(polymer: &mut HashMap<String, usize>, char_counter: &mut HashMap<char, usize>, insertion_rules: &HashMap<String, String>) {
    for (pair, count) in polymer.clone().iter() {
        // Skip if we should not be inserting for this pair
        if !insertion_rules.contains_key(pair) {
            continue
        }
        let insert_char = insertion_rules.get(pair).unwrap();
        let (a, b) = pair.split_at(1);
        *polymer.entry(pair.clone()).or_insert(0) -= count;
        let new_pair_one = String::from_iter([a.to_string(), insert_char.clone()]);
        *polymer.entry(new_pair_one).or_insert(0) += count;
        let new_pair_two = String::from_iter([insert_char.clone(), b.to_string()]);
        *polymer.entry(new_pair_two).or_insert(0) += count;
        *char_counter.entry(insert_char.chars().nth(0).unwrap()).or_insert(0) += count;
    }
}

fn solve_one(contents: &String) -> usize {
    let now = Instant::now();
    let sol = run(10, contents);
    println!("Solve one score: {}", sol);
    let elapsed_time = now.elapsed();
    println!("Running solve_one() took {:?} ms.", elapsed_time);
    sol
}

fn solve_two(contents: &String) -> usize {
    let now = Instant::now();
    let sol = run(40, contents);
    println!("Solve two score: {}", sol);
    let elapsed_time = now.elapsed();
    println!("Running solve_two() took {:?} ms.", elapsed_time);
    sol
}


const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_one() {
        assert_eq!(solve_one(&TEST_INPUT.to_string()), 1588)
    }

    #[test]
    fn test_two() {
        assert_eq!(solve_two(&TEST_INPUT.to_string()), 2188189693529)
    }
}