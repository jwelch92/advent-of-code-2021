use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    count_measurements(&contents);
    count_in_windows(&contents);
}

fn count_measurements(contents: &String) {
    let mut previous: u16 = u16::MAX;
    let mut increased: u16 = 0;

    contents.split("\n").for_each({
        |x| match x.parse::<u16>() {
            Ok(x) => {
                if x > previous {
                    increased += 1;
                }
                previous = x;
            },
            Err(_) => ()
        }
    });
    println!("There are {} measurements greater than the previous", increased);
}

fn count_in_windows(contents: &String) {
    let lines: Vec<&str> = contents.lines().collect();
    let mut last_window: u16 = u16::MAX;
    let mut increased_windows: u16 = 0;
    lines.windows(3).for_each({
        |x| {
            let window_total: u16 = x.iter().map(|y| y.parse::<u16>().unwrap() ).sum();
            if window_total > last_window {
                increased_windows += 1;
            }
            last_window = window_total;
        }
    });

    println!("There are {} measurement windows greater than the previous", increased_windows);
}