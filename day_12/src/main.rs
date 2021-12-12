use std::borrow::Borrow;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    solve_one(&contents);
    solve_two(&contents);
}

#[derive(Debug)]
struct Graph {
    adj_list: HashMap<String, Vec<String>>
}

impl Graph {
    fn new() -> Self {
        Graph {
            adj_list: HashMap::new()
        }
    }

    fn insert_edge(&mut self, line: &str) {
        let (a, b) = line.split_once("-").unwrap();
        self.adj_list.entry(String::from(a)).or_insert_with(Vec::new).push(String::from(b));
        self.adj_list.entry(String::from(b)).or_insert_with(Vec::new).push(String::from(a));
    }

    fn find_paths(&self, seen: &mut HashSet<String>, cave: &String) -> usize {
        println!("On cave {}", cave);
        if cave == &"end" {
            println!("Hit the end for this path! {}", cave);
            return 1;
        }
        let mut out: usize = 0;
        for next_cave in self.adj_list.get(cave).unwrap().iter() {
           if next_cave.chars().all(|c| c.is_lowercase()) {
               if !seen.contains(next_cave) {
                   seen.insert(next_cave.clone());
                   out += self.find_paths(seen, next_cave);
               }
           } else {
               out += self.find_paths(seen, next_cave);
           }
        }
        out
    }

    fn count_paths_non_recur(&self) -> usize {
        let mut queue: VecDeque<Vec<&String>> = VecDeque::new();
        queue.push_back(vec![&String::from("start")]);
        let mut count = 0;

        // Try using deq soln as the recursive one does not seem to work for me :/
    }

}




fn solve_one(contents: &String) -> usize {
    let mut graph = Graph::new();
    for line in contents.trim().lines() {
        graph.insert_edge(line);
    }

    println!("{:?}", graph.adj_list);
    let mut seen: HashSet<String> = HashSet::new();
    // seen.insert(String::from("start"));
    graph.find_paths(&mut seen, &String::from("start"))
}

fn solve_two(contents: &String) -> usize {
    todo!()
}

const TEST_INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_one() {
        assert_eq!(solve_one(&TEST_INPUT.to_string()), 10)
    }

    #[test]
    fn test_two() {
        assert_eq!(solve_two(&TEST_INPUT.to_string()), 195)
    }
}


