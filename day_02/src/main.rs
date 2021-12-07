use std::fs;

#[derive(Debug, Default)]
struct Position {
    aim: u32,
    depth: u32,
    horizonal_pos: u32,
}

impl Position {
    fn new() -> Self {
        Default::default()
    }

    fn solve(&self) -> u32 {
        self.horizonal_pos * self.depth
    }
}

#[derive(Debug, Clone)]
struct Move {
    kind: MoveKind,
    unit: u32
}

impl Move {
    fn new() -> Self {
        Default::default()
    }
}

impl Default for Move {
    fn default() -> Self {
        Move {
            kind: MoveKind::Forward,
            unit: 0,
        }
    }
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let mut iter = input.split(" ");
        let mut mov = Move::new();

        mov.kind = match iter.next() {
            Some(maybe) => match maybe {
                "forward" => MoveKind::Forward,
                "down" => MoveKind::Down,
                "up" => MoveKind::Up,
                _ => {
                    panic!("Invalid move");
                }
            },
            None => panic!("Invalid input!")
        };

        mov.unit = match iter.next() {
            Some(maybe) => match maybe.parse() {
                Ok(unit) => unit,
                Err(_e) => panic!("Invalid unit!")
            },
            None => panic!("Invalid input")
        };

        return mov;
    }
}

#[derive(Debug, Clone)]
enum MoveKind {
    Forward,
    Down,
    Up
}


fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    println!("{}", contents);
    let mut pos_part_1 = Position::new();
    let mut pos_part_2 = Position::new();

    let moves: Vec<Move> = contents.lines().map(|x| Move::from(x)).collect();
    println!("{:?}", moves);

    for m in &moves {
        match m.kind {
            MoveKind::Forward => pos_part_1.horizonal_pos += m.unit,
            MoveKind::Up => pos_part_1.depth -= m.unit,
            MoveKind::Down => pos_part_1.depth += m.unit,
        }
    }
    println!("Part 1 {:?} final answer {}", pos_part_1, pos_part_1.solve());

    for m in &moves {
        match m.kind {
            MoveKind::Up => pos_part_2.aim -= m.unit,
            MoveKind::Down => pos_part_2.aim += m.unit,
            MoveKind::Forward => {
                pos_part_2.horizonal_pos += m.unit;
                pos_part_2.depth += pos_part_2.aim * m.unit;
            },
        }
    }
    println!("Part 2 {:?} final answer {}", pos_part_2, pos_part_2.solve());

}
