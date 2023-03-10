use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const DIVISORS: [isize; 2] = [3, 5];
    const UPPER_LIMIT: isize = 999;

    let answer = sum_multiples(&DIVISORS, UPPER_LIMIT);

    println!("{answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

fn sum_multiples(divisors: &[isize], upper_limit: isize) -> isize {
    let mut s = 0;
    for i in 1..=upper_limit {
        for d in divisors {
            if i % d == 0 {
                s += i;
                break;
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use crate::sum_multiples;
    #[test]
    fn test_sum_multiples() {
        assert_eq!(sum_multiples(&[3, 5], 9), 23);
    }
}
