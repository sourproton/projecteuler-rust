use std::time::SystemTime;

const DIVISORS: [u32; 2] = [3, 5];
const UPPER_LIMIT: u32 = 999;

fn main() {
    let time = SystemTime::now();

    let answer = sum_multiples(&DIVISORS, UPPER_LIMIT);

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

/// From 1 to `upper_limit`, returns the sum of all that are divisible by at least one of the
/// `divisors`
fn sum_multiples(divisors: &[u32], upper_limit: u32) -> u32 {
    (1..=upper_limit)
        .filter(|n| divisors.iter().any(|d| n % d == 0))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::sum_multiples;

    #[test]
    fn test_sum_multiples() {
        assert_eq!(sum_multiples(&[3, 5], 9), 23);
    }
}
