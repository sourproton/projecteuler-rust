use std::time::SystemTime;

fn main() {
    let time = SystemTime::now();

    const N: usize = 2_000_000;
    let mut sieve = [true; N];
    let mut answer = 0;

    for i in 2..=N {
        if sieve[i - 1] {
            answer += i;
            for j in (i + i..=N).step_by(i) {
                sieve[j - 1] = false;
            }
        }
    }

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}
