use std::time::SystemTime;

const PERIMETER_LIMIT: u64 = 1_000_000_000;

fn main() {
    let time = SystemTime::now();

    let answer = sum_perimeters(PERIMETER_LIMIT);

    println!("answer: {answer:#?}");
    println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
}

/// Sum of the perimeters of all almost equilateral triangles whose areas are integral and whose
/// repeated sized sides aren't bigger than `double_limit`
fn sum_perimeters(perimeter_limit: u64) -> u64 {
    let double_limit = (perimeter_limit + 1) / 3;

    let mut s = 0;

    for d in 1..=double_limit {
        let triangle_1 = AlmostEquilateral {
            double: d,
            single: d - 1,
        };

        let per_1 = triangle_1.perimeter();

        if triangle_1.single > 0 && per_1 <= PERIMETER_LIMIT && triangle_1.area_is_integral() {
            s += per_1;
        }

        let triangle_2 = AlmostEquilateral {
            double: d,
            single: d + 1,
        };

        let per_2 = triangle_2.perimeter();

        if per_2 <= PERIMETER_LIMIT && triangle_2.area_is_integral() {
            s += per_2;
        }
    }

    s
}

/// Descibres an almost equilateral triangle, which has 2 sides of `double` length, and one side
/// of `single` length
#[derive(Debug)]
struct AlmostEquilateral {
    double: u64,
    single: u64,
}

impl AlmostEquilateral {
    fn area_is_integral(&self) -> bool {
        let d_squared = (self.double as f64).powi(2);
        let s_squared = (self.single as f64).powi(2);

        let area_numerator = (4.0 * d_squared * s_squared - s_squared * s_squared).sqrt();

        area_numerator.fract() == 0.0
    }

    fn perimeter(&self) -> u64 {
        2 * self.double + self.single
    }
}

#[cfg(test)]
mod tests {
    use crate::AlmostEquilateral;

    const TEST_TRIANGLE: AlmostEquilateral = AlmostEquilateral {
        double: 5,
        single: 6,
    };

    #[test]
    fn test_area_is_integral() {
        assert!(TEST_TRIANGLE.area_is_integral());
    }

    #[test]
    fn test_perimeter() {
        assert_eq!(16, TEST_TRIANGLE.perimeter());
    }
}
