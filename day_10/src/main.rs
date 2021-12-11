use std::collections::HashMap;
use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    solve_one(&contents);
    solve_two(&contents);
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

fn process_input(contents: &String) -> (Vec<char>, Vec<Vec<char>>) {
    let mut invalid_chars: Vec<char> = vec![];
    let mut incomplete_lines: Vec<Vec<char>> = vec![];
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
                        }
                        Some('(') => {
                            if c != ')' {
                                println!("Found fault! at index={}, expected {} but got {}", i, ')', c);
                                invalid_chars.push(c);
                            }
                        }
                        Some('{') => {
                            if c != '}' {
                                println!("Found fault! at index={}, expected {} but got {}", i, '}', c);
                                invalid_chars.push(c);
                            }
                        }
                        Some('<') => {
                            if c != '>' {
                                println!("Found fault! at index={}, expected {} but got {}", i, '>', c);
                                invalid_chars.push(c);
                            }
                        }
                        // Weird input, panic out
                        _ => panic!("Wtf!")
                    }
                }
                // again, weird input panic out and satisfy the match completeness
                _ => panic!("WTF!")
            }
        }
        // If chunks is not empty then we had unclosed opening chars
        println!("Chunks left: {:?}", chunks);
        if !chunks.is_empty() {
            incomplete_lines.push(chunks);
        }
    }
    (invalid_chars, incomplete_lines)
}

fn solve_one(contents: &String) -> usize {
    let (invalid_chars, _) = process_input(contents);
    let score: usize = invalid_chars.iter().fold(0, |acc, x| {
        acc + score_char(*x)
    });
    println!("Total score for corrupted lines: {}", score);
    score
}

fn score_added_chars(added: Vec<char>) -> usize {
    added.iter().fold(0, |acc, x| {
        // chars are opposite because we're scoring from the opening chars only
        let increase = match x {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0,
        };
        (acc * 5) + increase
    })
}


fn solve_two(contents: &String) -> usize {
    // Discard corrupted lines so skip if we don't find a matching pair
    // Instead push all chars
    // Just return each Left over chunks and complete the stack!!!! See above on line 67!
    let (_, incomplete_chunks) = process_input(contents);
    println!("{:?}", incomplete_chunks);
    let mut scores: Vec<usize> = vec![];
    for mut chunk in incomplete_chunks {
        // reverse chunk so we score in the right order
        chunk.reverse();
        println!("Chunk reversed: {:?}", chunk);
        let sc = score_added_chars(chunk);
        println!("Score: {}", sc);
        scores.push(sc);
    }
    scores.sort();
    // get midpoint. task says there will always be an odd number of incomplete lines
    let final_score = scores[scores.len() / 2];
    println!("Final score for part two: {}", final_score);
    final_score
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
    assert_eq!(solve_two(&TEST_INPUT.to_string()), 288957)
}

#[test]
fn test_part_scorer() {
    assert_eq!(score_added_chars(vec![']', ')', '}', '>']), 294)
}