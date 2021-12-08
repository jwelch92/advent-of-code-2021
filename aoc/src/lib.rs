#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub mod aoc {
    use std::fs;

    pub fn read_input(file_name: &str) -> String {
        fs::read_to_string(file_name).expect("Error opening file")
    }
}