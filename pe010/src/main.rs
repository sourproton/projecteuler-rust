fn main() {
    const N: usize = 2_000_000;
    let mut sieve = [true; N];
    let mut s = 0;

    for i in 2..=N {
        if sieve[i - 1] {
            s += i;
            for j in (i + i..=N).step_by(i) {
                sieve[j - 1] = false;
            }
        }
    }

    println!("{s}");
}
