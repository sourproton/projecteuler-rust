use std::time::SystemTime;

const LIMIT: u64 = 100;

fn main() {
    let time = SystemTime::now();

    let answer = diff(LIMIT);

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

/// calculates the difference between the square of the sum and the sum of the squares of integers
/// from 1 to `n`
fn diff(limit: u64) -> u64 {
    let mut s = 0;
    let mut s_sq = 0;

    for n in 1..=limit {
        s += n;
        s_sq += n.pow(2);
    }

    s.pow(2) - s_sq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff() {
        let answer = diff(10);
        assert_eq!(2640, answer);
    }
}
