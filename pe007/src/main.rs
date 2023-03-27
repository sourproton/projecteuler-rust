use std::time::SystemTime;

// const N: u32 = 1_000_000;
const N: u32 = 10_001;

fn main() {
    let time = SystemTime::now();

    let answer = nth_prime(N);

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

fn nth_prime(n: u32) -> u32 {
    let mut primes = Vec::from([2]);

    while (primes.len() as u32) < n {
        push_next_prime(&mut primes);
    }

    *primes.last().unwrap()
}

fn push_next_prime(primes: &mut Vec<u32>) {
    let mut candidate = primes.last().unwrap().to_owned() + 1;
    let mut max_prime = (candidate as f32).sqrt() as u32;

    while primes
        .iter()
        .take_while(|p| **p <= max_prime)
        .any(|p| candidate % p == 0)
    {
        candidate += 1;
        max_prime = (candidate as f32).sqrt() as u32;
    }

    primes.push(candidate);
}

#[cfg(test)]
mod tests {
    use crate::nth_prime;

    #[test]
    fn test_nth_prime() {
        assert_eq!(13, nth_prime(6));
    }
}
