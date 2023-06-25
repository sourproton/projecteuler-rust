use std::time::SystemTime;

const POWER: usize = 1000;

fn main() {
    let time = SystemTime::now();

    let answer: usize = power_two(POWER).iter().sum();

    println!("answer: {answer}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

/// returns the `n`th power of two in "vector-number" form
fn power_two(n: usize) -> Vec<usize> {
    let mut v = vec![2];

    for _ in 1..=n - 1 {
        times_two(&mut v);
    }

    v
}

/// multiplies a "vector-number" by two
fn times_two(v: &mut Vec<usize>) {
    // quocient
    let mut q = 0;
    //remainder
    let mut r = 0;

    v.iter_mut().for_each(|d| {
        *d *= 2;
        *d += q;
        (q, r) = (*d / 10, *d % 10);
        *d = r;
    });

    if q > 0 {
        v.push(q)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_times_two() {
        let mut v = vec![1, 2, 3, 4, 5];
        times_two(&mut v);
        assert_eq!(vec![2, 4, 6, 8, 0, 1], v);
    }

    #[test]
    fn test_power_two() {
        let v = power_two(15);
        assert_eq!(vec![8, 6, 7, 2, 3], v);
    }
}
