use std::time::SystemTime;

const LIMIT: usize = 9_999;

fn main() {
    let time = SystemTime::now();

    let answer = sum_amicables(LIMIT);

    println!("sum of amicable numbers below {LIMIT} is {answer}");

    println!("elapsed time = {} ms", time.elapsed().unwrap().as_millis());
}

fn sum_amicables(limit: usize) -> usize {
    let mut amicables = vec![false; limit];
    let mut s = 0;
    let (mut b, mut db);

    for a in 1..=limit {
        if !amicables[a - 1] {
            b = sum_of_divisors(a);
            if a < b {
                db = sum_of_divisors(b);
                if db == a {
                    amicables[a - 1] = true;
                    amicables[b - 1] = true;
                    s += a + b;
                }
            }
        }
    }

    s
}

fn sum_of_divisors(n: usize) -> usize {
    (1..=(n / 2)).filter(|x| n % x == 0).sum()
}

#[cfg(test)]
mod tests {
    use crate::sum_of_divisors;

    #[test]
    fn test_sum_of_divisors() {
        let d220 = sum_of_divisors(220);
        assert_eq!(284, d220);

        let d284 = sum_of_divisors(284);
        assert_eq!(220, d284);
    }
}
