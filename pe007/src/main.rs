fn nthprime(n: i32) -> i32 {
    let mut primes = Vec::new();
    let mut i = 2;
    let mut found = 0;
    let mut isprime = true;

    while found < n {
        for p in &primes {
            if i % p == 0 {
                isprime = false;
                break;
            }
        }

        if isprime {
            found += 1;
            primes.push(i);
        } else {
            isprime = true;
        }

        i += 1;
    }

    primes[n as usize - 1]
}

fn main() {
    const NUM: i32 = 10001;
    
    let answer = nthprime(NUM);

    println!("{answer}");
}
