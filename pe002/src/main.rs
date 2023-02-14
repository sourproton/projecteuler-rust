fn main() {
    const LIMIT: isize = 4_000_000;

    let answer = sum_even_fib(LIMIT);

    println!("{answer}");
}

fn update_fib(fib: &mut (isize, isize)) {
    (fib.0, fib.1) = (fib.1, fib.0 + fib.1);
}

fn sum_even_fib(limit: isize) -> isize {
    let mut fib = (1, 2);
    let mut s = 0;

    while fib.1 < limit {
        if fib.1 % 2 == 0 {
            s += fib.1;
        }
        update_fib(&mut fib);
    }

    s
}

#[cfg(test)]
mod tests {
    use crate::sum_even_fib;
    #[test]
    fn test_sum_multiples() {
        assert_eq!(sum_even_fib(89), 2 + 8 + 34);
    }
}
