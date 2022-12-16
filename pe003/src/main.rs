fn main() {
    const NUM: u64 = 600851475143;
    let answer = largest_prime_factor(NUM);
    println!("{}", answer);
}

fn largest_prime_factor(mut n: u64) -> u64 {
    let mut test = 2;
    let mut largest = 0;
    while n != 1 {
       while n % test == 0 {
           largest = test;
           n = n / test;
       } 
       test += 1;
    }
    largest
}
