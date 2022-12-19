fn number_of_divisors(mut n: usize) -> usize {
    let mut ndivisors = 1;
    let mut divisor = 2;
    let mut count;

    while n != 1 {
        count = 0;
        while n % divisor == 0 {
            count += 1;
            n /= divisor;
        }
        divisor += 1;
        ndivisors *= count + 1;
    }

    ndivisors
}

fn triangle_ndivisors(n: usize) -> usize {
    let mut i = 1;
    let mut triangle = 1;

    while number_of_divisors(triangle) < n {
        i += 1;
        triangle += i;
    }

    triangle
}

fn main() {
    const NUM: usize = 500;
    println!("{}", triangle_ndivisors(NUM));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_divisors() {
        assert_eq!(number_of_divisors(1), 1);
        assert_eq!(number_of_divisors(10), 4);
        assert_eq!(number_of_divisors(28), 6);
    }

    #[test]
    fn test_triangle_ndivisors() {
        assert_eq!(triangle_ndivisors(5), 28);
    }
}
