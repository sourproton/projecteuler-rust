use std::time::SystemTime;

const TARGET_FRACTION: Fraction = Fraction {
    numerator: 3,
    denominator: 7,
};

const MAX_DENOMINATOR: u32 = 1_000_000;

fn main() {
    let time = SystemTime::now();

    let answer = find_closest(MAX_DENOMINATOR, TARGET_FRACTION);

    println!("closest fraction: {:#?}", answer);

    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

fn find_closest(max_denominator: u32, target_fraction: Fraction) -> Fraction {
    let mut best_fraction = Fraction {
        numerator: 0,
        denominator: 1,
    };

    let mut numerator;

    for denominator in 2..=max_denominator {
        numerator = find_numerator(&denominator, &target_fraction);
        if best_fraction.numerator * denominator < numerator * best_fraction.denominator {
            best_fraction.numerator = numerator;
            best_fraction.denominator = denominator;
        }
    }

    best_fraction
}

fn find_numerator(denominator: &u32, target_fraction: &Fraction) -> u32 {
    (denominator * target_fraction.numerator - 1) / (target_fraction.denominator)
}

#[derive(Debug)]
struct Fraction {
    numerator: u32,
    denominator: u32,
}

#[cfg(test)]
mod tests {
    use crate::{find_closest, Fraction};

    #[test]
    fn test_find_closest() {
        let closest = find_closest(
            8,
            Fraction {
                numerator: 3,
                denominator: 7,
            },
        );
        assert_eq!(2, closest.numerator);
        assert_eq!(5, closest.denominator);
    }
}
