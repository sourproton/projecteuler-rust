use std::time::SystemTime;

const LIMIT: usize = 28123;

fn main() {
    let time = SystemTime::now();

    let answer = sum_not_sum_two_abundants();

    println!("answer: {answer}");
    println!("time elapsed: {} ms", time.elapsed().unwrap().as_millis());
}

fn sum_not_sum_two_abundants() -> usize {
    let is_sum_two_abundants = vec_sum_two_abundants();
    is_sum_two_abundants
        .iter()
        .enumerate()
        .map(|(i, val)| if !*val { i + 1 } else { 0 })
        .sum()
}

fn vec_sum_two_abundants() -> [bool; LIMIT] {
    let mut sum_two_abundants = [false; LIMIT];
    let abundants = vec_abundants();
    let mut s;

    for i in 0..abundants.len() {
        for j in i..abundants.len() {
            s = abundants[i] + abundants[j];
            if s > LIMIT {
                break;
            }
            sum_two_abundants[s - 1] = true;
        }
    }

    sum_two_abundants
}

fn vec_abundants() -> Vec<usize> {
    (12..=LIMIT).filter(|x| is_abundant(*x)).collect()
}

fn is_abundant(n: usize) -> bool {
    sum_proper_divisors(n) > n
}

fn sum_proper_divisors(n: usize) -> usize {
    (1..=(n / 2)).filter(|x| n % x == 0).sum()
}

#[cfg(test)]
mod tests {
    use crate::is_abundant;

    #[test]
    fn unit_023() {
        assert_eq!(true, is_abundant(12))
    }
}
