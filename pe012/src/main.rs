fn main() {
    const N: usize = 501;

    let answer = lowest_triangular(N);

    println!("{answer}");
}

fn nth_triangular(n: usize) -> usize {
    let mut s = 0;

    for i in 1..=n {
        s += i;
    }

    s
}

fn ndivisors(n: usize) -> usize {
    let mut count = 1; // 1 and n itself are divisors

    for i in 1..=n / 2 {
        if n % i == 0 {
            count += 1;
        }
    }

    count
}

fn lowest_triangular(nd: usize) -> usize {
    let mut i = 1;
    let mut t = i;

    while ndivisors(t) < nd {
        i += 1;
        t += i;
    }

    t
}

#[cfg(test)]
mod tests {
    use crate::{lowest_triangular, ndivisors, nth_triangular};

    #[test]
    fn working_nth_triangular() {
        let t = nth_triangular(7);
        assert_eq!(t, 28)
    }

    #[test]
    fn working_ndivisors() {
        assert_eq!(ndivisors(1), 1);
        assert_eq!(ndivisors(3), 2);
        assert_eq!(ndivisors(6), 4);
        assert_eq!(ndivisors(10), 4);
        assert_eq!(ndivisors(15), 4);
        assert_eq!(ndivisors(21), 4);
        assert_eq!(ndivisors(28), 6);
    }

    #[test]
    fn working_lowest_triangular() {
        assert_eq!(lowest_triangular(6), 28)
    }
}
