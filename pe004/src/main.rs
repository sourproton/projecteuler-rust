use std::time::SystemTime;

const N: u32 = 3;

fn main() {
    let time = SystemTime::now();

    let answer = greatest_palindrome(N);

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

/// The greatest palindrome that is the product of two `n`-digit numbers
fn greatest_palindrome(n: u32) -> usize {
    let low = 10_usize.pow(n - 1);
    let high = 10_usize.pow(n) - 1;

    let mut highest = 0;

    for i in low..=high {
        for j in low..=high {
            let p = i * j;
            if is_palindrome(p) {
                highest = highest.max(p);
            }
        }
    }

    highest
}

/// Wether a number is palindrome or not
fn is_palindrome(n: usize) -> bool {
    // let n_str = n.to_string();
    // let forward = n_str.chars();
    // let backward = n_str.chars().rev();
    // !forward.zip(backward).any(|(a, b)| a != b)

    let (mut rev, mut org) = (0, n);

    while org > 0 {
        rev = (rev * 10) + org % 10;
        org /= 10;
    }

    rev == n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(9009));
        assert!(is_palindrome(90109));
    }

    #[test]
    fn test_greatest() {
        assert_eq!(9009, greatest_palindrome(2));
    }
}
