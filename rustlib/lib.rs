use std::fs;

pub mod utils {
    use super::*;
    pub fn get_file_contents(file_path: &str) -> String {
        fs::read_to_string(file_path)
            .unwrap()
            .replace("\r\n", "\n")
            .replace("\r", "\n")
    }
    pub fn get_lines(file_path: &str) -> Vec<String> {
        get_file_contents(file_path)
            .lines()
            .map(ToOwned::to_owned)
            .collect()
    }

    pub fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    pub fn lcm(a: usize, b: usize) -> usize {
        (a * b) / gcd(a, b)
    }

    pub fn lcm_of_vec(numbers: &[usize]) -> usize {
        numbers.iter().cloned().fold(1, |acc, num| lcm(acc, num))
    }
}
