use std::time::SystemTime;

const S: u32 = 1000;

fn main() {
    let time = SystemTime::now();

    if let Some((a, b, c)) = find_triplet(S) {
        println!("a = {a}, b = {b}, c = {c}");
        println!("answer: a*b*c = {}", a * b * c);
        println!("elapsed time: {} ms", time.elapsed().unwrap().as_millis());
    }
}

/// Returns the first Pythagorean triplet found with sum `s`
fn find_triplet(s: u32) -> Option<(u32, u32, u32)> {
    for a in 1..=s {
        for b in a + 1..=s {
            let c = s - a - b;
            if a * a + b * b == c * c {
                return Some((a, b, c));
            }
        }
    }

    None
}
