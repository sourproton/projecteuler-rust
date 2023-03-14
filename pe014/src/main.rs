use std::time::SystemTime;

const LOW: usize = 1;
const HIGH: usize = 999_999;

fn main() {
    let time = SystemTime::now();

    let longest = longest_collatz(LOW, HIGH);

    let answer = longest.number;

    println!("longest found: {longest:#?}");

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

fn longest_collatz(low: usize, high: usize) -> Collatz {
    let mut longest = Collatz {
        number: low,
        length: collatz_length(low),
    };

    for n in (low + 1)..=high {
        let l = collatz_length(n);
        if l > longest.length {
            longest.number = n;
            longest.length = l;
        }
    }

    longest
}

fn collatz_length(mut n: usize) -> usize {
    let mut l = 1;

    while n != 1 {
        l += 1;
        n = next_collatz(n);
    }

    l
}

fn next_collatz(n: usize) -> usize {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

#[derive(Debug)]
struct Collatz {
    number: usize,
    length: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_length() {
        let l = collatz_length(13);
        assert_eq!(l, 10)
    }
}
