use std::time::SystemTime;

const FACTORIAL: usize = 100;

fn main() {
    let time = SystemTime::now();

    let answer = sum_digits_factorial(FACTORIAL);

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

/// returns the sum of the digits of `n`th factorial
fn sum_digits_factorial(n: usize) -> usize {
    factorial(n).iter().sum()
}

/// returns the factorial of `n` in "vector-number" form
fn factorial(n: usize) -> Vec<usize> {
    let mut v = vec![2];

    for n in 3..=n {
        times_n(n, &mut v);
    }

    v
}

/// multiplies a "vector-number" by `n`
fn times_n(n: usize, v: &mut Vec<usize>) {
    // quocient
    let mut q = 0;
    //remainder
    let mut r = 0;

    v.iter_mut().for_each(|d| {
        *d *= n;
        *d += q;
        (q, r) = (*d / 10, *d % 10);
        *d = r;
    });

    while q != 0 {
        v.push(q % 10);
        q /= 10;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_times_n() {
        let mut v = vec![1, 2, 3, 4, 5];
        times_n(2, &mut v);
        assert_eq!(vec![2, 4, 6, 8, 0, 1], v);
    }

    #[test]
    fn test_factorial() {
        let v = factorial(10);
        assert_eq!(vec![0, 0, 8, 8, 2, 6, 3], v);
    }

    #[test]
    fn test_sum_digits() {
        let s = sum_digits_factorial(10);
        assert_eq!(27, s);
    }
}
