use std::time::SystemTime;

const A_RANGE: (i32, i32) = (-999, 999);
const B_RANGE: (i32, i32) = (-1_000, 1_000);

fn main() {
    let time = SystemTime::now();

    let max = find_coefficients(A_RANGE, B_RANGE);

    let answer = max.a * max.b;

    println!("max found: {max:#?}");

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

fn find_coefficients(a_range: (i32, i32), b_range: (i32, i32)) -> Coefficients {
    let mut max = Coefficients { n: 0, a: 0, b: 0 };
    let mut n;
    for a in a_range.0..=a_range.1 {
        for b in b_range.0..=b_range.1 {
            n = max_consecutive_primes(a, b);
            if n > max.n {
                max.n = n;
                max.a = a;
                max.b = b;
            }
        }
    }
    max
}

fn max_consecutive_primes(a: i32, b: i32) -> i32 {
    let mut n = 0;
    loop {
        if is_prime(quadratic(n, a, b)) {
            n += 1;
        } else {
            return n - 1;
        }
    }
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    let limit = (n as f32).sqrt().floor() as i32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn quadratic(n: i32, a: i32, b: i32) -> i32 {
    n * n + a * n + b
}

#[derive(Debug)]
struct Coefficients {
    n: i32,
    a: i32,
    b: i32,
}

#[cfg(test)]
mod tests {
    use crate::{is_prime, max_consecutive_primes};

    #[test]
    fn test_is_prime() {
        assert_eq!(false, is_prime(0));
        assert_eq!(false, is_prime(1));
        assert_eq!(true, is_prime(2));
        assert_eq!(true, is_prime(3));
        assert_eq!(false, is_prime(4));
    }

    #[test]
    fn test_count() {
        let mut x = max_consecutive_primes(1, 41);
        assert_eq!(39, x);
        x = max_consecutive_primes(-79, 1601);
        assert_eq!(79, x);
    }
}
