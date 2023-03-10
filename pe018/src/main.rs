use std::{cmp::max, fs::read_to_string};

const PATH: &str = "input/input_018.txt";

fn main() {
    let max = maximum_path_sum(parse_input(PATH));
    println!("max path sum = {}", max);
}

fn maximum_path_sum(mut input: Vec<Vec<usize>>) -> usize {
    for l in (0..input.len() - 2).rev() {
        for i in 0..input[l].len() {
            input[l][i] += max(input[l + 1][i], input[l + 1][i + 1])
        }
    }
    input[0][0]
}

fn parse_input(path: &str) -> Vec<Vec<usize>> {
    read_to_string(path)
        .unwrap()
        .split('\n')
        .map(|x| parse_line(x))
        .collect()
}

fn parse_line(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{maximum_path_sum, parse_input};

    #[test]
    fn test_maximum_path_sum() {
        let input = parse_input("./input/test_018.txt");
        println!("{:?}", input);
        let max = maximum_path_sum(input);
        assert_eq!(23, max);
    }
}
