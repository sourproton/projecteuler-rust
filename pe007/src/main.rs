fn main() {
    const N: usize = 10_001;
    let mut primes: [usize; N] = [0; N];
    let mut i = 2;

    while primes[N - 1] == 0 {
        for p in 0..N {
            if primes[p] == 0 {
                primes[p] = i;
                i += 1;
                break;
            } else if i % primes[p] == 0 {
                i += 1;
                break;
            }
        }
    }

    println!("{}", primes[N - 1]);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn working() {
//         assert_eq!(2 + 2, 4)
//     }
// }
