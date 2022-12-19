fn next_collatz(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn collatz_length(mut n: u64) -> u64 {
    let mut l = 1;

    while n != 1 {
        l += 1;
        n = next_collatz(n);
    }

    l
}

fn longest_collatz_below(n: u64) -> u64 {
    let mut longest = (1, collatz_length(1));
    let mut l;

    for i in 2..n {
        l = collatz_length(i);
        if l > longest.1 {
            (longest.0, longest.1) = (i, l);
        }
    }

    longest.0
}

fn main() {
    const NUM: u64 = 1_000_000;
    println!("{}", longest_collatz_below(NUM));
}
