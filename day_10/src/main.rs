use std::collections::HashMap;
use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    solve_one(&contents);
    // solve_two(&contents);
}

fn score_char(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid char used {}", c)
    }
}

fn solve_one(contents: &String) -> usize {
    let mut invalid_chars: Vec<char> = vec![];
    for (lineno, line) in contents.trim().lines().enumerate() {
        println!("Checking line {}", lineno);
        println!("{}", line);
        let mut chunks: Vec<char> = vec![];
        for (i, c) in line.trim().chars().enumerate() {
            // println!("Chunks {:?}", chunks);
            match c {
                '[' | '(' | '{' | '<' => chunks.push(c),
                ']' | ')' | '}' | '>' => {
                    let popped: Option<char> = chunks.pop();
                    // println!("Popped={:?} C ='{}'", popped, c);
                    match popped {
                        Some('[') => {
                            if c != ']' {
                                println!("Found fault! at index={}, expected {} but got {}", i, ']', c);
                                invalid_chars.push(c);
                            }
                        },
                        Some('(') => {
                            if c != ')' {
                                println!("Found fault! at index={}, expected {} but got {}", i, ')', c);
                                invalid_chars.push(c);
                            }
                        },
                        Some('{') => {
                            if c != '}' {
                                println!("Found fault! at index={}, expected {} but got {}", i, '}', c);
                                invalid_chars.push(c);
                            }
                        },
                        Some('<') => {
                            if c != '>' {
                                println!("Found fault! at index={}, expected {} but got {}", i, '>', c);
                                invalid_chars.push(c);
                            }
                        },
                        _ => panic!("Wtf!")
                    }
                },
                _ => panic!("WTF!")

            }
        }
    }
    let score: usize = invalid_chars.iter().fold(0, |acc, x| {
        acc + score_char(*x)
    });
    println!("Total score for corrupted lines: {}", score);
    score
}

fn solve_two(contents: &String) -> usize {
    todo!();
    0
}


const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

#[cfg(test)]
#[test]
fn test_one() {
    assert_eq!(solve_one(&TEST_INPUT.to_string()), 26397)
}

#[test]
fn test_two() {
    assert_eq!(solve_two(&TEST_INPUT.to_string()), 0)
}