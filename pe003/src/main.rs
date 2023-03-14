use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const N: isize = 600851475143;

    let answer = largest_prime_factor(N);

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

fn largest_prime_factor(mut n: isize) -> isize {
    let mut p = 2;

    while p != n {
        while n % p == 0 {
            n /= p;
        }
        p += 1;
    }

    p
}

#[cfg(test)]
mod tests {
    use crate::largest_prime_factor;
    #[test]
    fn it_works() {
        let result = largest_prime_factor(13195);
        assert_eq!(result, 29);
    }
}
