use std::time::SystemTime;

const N_DIVISORS: usize = 501;

fn main() {
    let time = SystemTime::now();

    let answer = lowest_triangular(N_DIVISORS);

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

/// returns the lowest triangular number to have at least n divisors
fn lowest_triangular(n: usize) -> usize {
    let mut t = 1;
    let mut i = 1;

    while count_divisors(t) < n {
        i += 1;
        t += i;
    }

    t
}

/// returns the number of divisors of n, calculated via the factorization method
fn count_divisors(mut n: usize) -> usize {
    let mut number_divisors = 1;
    let mut p = 2;
    let mut count;

    while n != 1 {
        count = 0;
        while n % p == 0 {
            count += 1;
            n /= p;
        }
        number_divisors *= count + 1;
        p += 1;
    }

    number_divisors
}

#[cfg(test)]
mod tests {
    use crate::{count_divisors, lowest_triangular};

    #[test]
    fn test_count_divisors() {
        assert_eq!(count_divisors(1), 1);
        assert_eq!(count_divisors(3), 2);
        assert_eq!(count_divisors(6), 4);
        assert_eq!(count_divisors(10), 4);
        assert_eq!(count_divisors(15), 4);
        assert_eq!(count_divisors(21), 4);
        assert_eq!(count_divisors(28), 6);
    }

    #[test]
    fn test_lowest_triangular() {
        assert_eq!(lowest_triangular(6), 28)
    }
}
