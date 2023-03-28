use std::time::SystemTime;

const LIMIT: u32 = 4_000_000;

fn main() {
    let time = SystemTime::now();

    let answer = Fib::sum_even_until(LIMIT);

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

/// Holds a Fibonacci pair
struct Fib {
    first: u32,
    second: u32,
}

impl Fib {
    /// Stating from `Fib { 1, 2 }`, sums all the pair Fibonacci numbers up to `limit`
    fn sum_even_until(limit: u32) -> u32 {
        let mut fib = Fib::start();
        let mut s = 0;

        while fib.second < limit {
            if fib.second % 2 == 0 {
                s += fib.second;
            }
            fib.update();
        }

        s
    }

    /// Returns the first Fibonacci pair (1, 2)
    fn start() -> Self {
        Fib {
            first: 1,
            second: 2,
        }
    }

    /// Updated the Fibonacci pair to its next iteration on the sequence
    fn update(&mut self) {
        (self.first, self.second) = (self.second, self.first + self.second);
    }
}

#[cfg(test)]
mod tests {
    use crate::Fib;

    #[test]
    fn test_sum_multiples() {
        assert_eq!(Fib::sum_even_until(89), 2 + 8 + 34);
    }
}
