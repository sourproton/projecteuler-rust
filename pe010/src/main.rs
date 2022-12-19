fn sum_prime(n: usize) -> usize {
    // sieve is a vector corresponding to 2 up to n
    let mut sieve = vec![true; n-1];
    let mut sum = 0;

    for num in 2..=n {
        if sieve[num-2] {
            sum += num;

            for i in (2*num..=n).step_by(num) {
                if sieve[i-2] {
                    sieve[i-2] = false;
                }
            }
        }
    }

    sum
}

fn main() {
    const NUM: usize = 1_999_999;
    println!("{}", sum_prime(NUM));
}
