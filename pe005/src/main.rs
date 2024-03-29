use std::{collections::HashMap, time::SystemTime};

const N: u32 = 20;

fn main() {
    let time = SystemTime::now();

    let answer = smallest_multiple(N);

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

/// calculates the smallest multiple of all numbers from 1 up to `n`
fn smallest_multiple(n: u32) -> u32 {
    // factorization of all numbers from 1 to n
    let factorizations = gen_factorizations(n);

    // picking the largest exponent from each base
    let combined = combined_factorizations(factorizations);

    // multiplying the bases raised to their powers
    combined.iter().map(|(k, v)| k.pow(*v)).product()
}

/// takes each base of each factorization and combines the biggest exponent of each in a HashMap
fn combined_factorizations(factorizations: HashMap<u32, HashMap<u32, u32>>) -> HashMap<u32, u32> {
    let mut combined: HashMap<u32, u32> = HashMap::new();

    for factorization in factorizations.values() {
        for (k, v) in factorization {
            combined
                .entry(*k)
                .and_modify(|e| *e = (*e).max(*v))
                .or_insert(*v);
        }
    }

    combined
}

/// returns a HashMap of numbers from 2 to `n` and their factorizations
fn gen_factorizations(n: u32) -> HashMap<u32, HashMap<u32, u32>> {
    let mut factorizations = HashMap::new();

    for i in 2..=n {
        factorizations.insert(i, factorize(i));
    }

    factorizations
}

/// returns the factorizaton of `n` in the form of a HashMap<factor, exponent>
fn factorize(mut n: u32) -> HashMap<u32, u32> {
    let mut factorization = HashMap::new();

    let mut factor = 2;

    while n != 1 {
        while n % factor == 0 {
            factorization
                .entry(factor)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            n /= factor;
        }
        factor += 1;
    }

    factorization
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{factorize, smallest_multiple};

    #[test]
    fn test_factorize() {
        let fac = factorize(60);
        let answer = HashMap::from([(2, 2), (3, 1), (5, 1)]);
        assert_eq!(answer, fac);
    }

    #[test]
    fn test_smallest_multiple() {
        assert_eq!(2520, smallest_multiple(10));
    }
}
