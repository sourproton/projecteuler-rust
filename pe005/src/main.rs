use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const LIMIT: isize = 20;

    let answer = min_divisible(LIMIT);

    println!("{answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

fn isdivisable(n: isize, limit: isize) -> bool {
    for i in 1..=limit {
        if n % i != 0 {
            return false;
        }
    }

    true
}

fn min_divisible(limit: isize) -> isize {
    let mut n = 1;

    while !(isdivisable(n, limit)) {
        n += 1;
    }

    n
}

#[cfg(test)]
mod tests {
    use crate::min_divisible;
    #[test]
    fn works() {
        let answer = min_divisible(10);
        assert_eq!(answer, 2520);
    }
}
