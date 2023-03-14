use std::{fs::read_to_string, time::SystemTime};

const PATH: &str = "./input/input_067.txt";

fn main() {
    let time = SystemTime::now();

    let mut input = parse_input(PATH);
    let answer = max_path_sum(&mut input);

    println!("answer: {answer}");
    println!("elapsed time = {} ms", time.elapsed().unwrap().as_millis());
}

fn max_path_sum(input: &mut Vec<Vec<u32>>) -> u32 {
    for row in (0..input.len() - 1).rev() {
        for col in 0..input[row].len() {
            input[row][col] += input[row + 1][col].max(input[row + 1][col + 1])
        }
    }
    input[0][0]
}

fn parse_input(path: &str) -> Vec<Vec<u32>> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{max_path_sum, parse_input};

    #[test]
    fn unit_test_pe067() {
        let path = "./input/test_067.txt";
        let mut input = parse_input(path);
        let answer = max_path_sum(&mut input);
        assert_eq!(23, answer);
    }
}
