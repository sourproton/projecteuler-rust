fn main() {
    let answer = difference(100);
    println!("{answer}");
}

fn difference(n: u32) -> u32 {
    let mut sum = 0;
    let mut sum_square = 0;

    for i in 1..=n {
        sum += i;
        sum_square += i * i;
    }

    sum * sum - sum_square
}
