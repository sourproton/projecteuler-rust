fn main() {
    let mut fib = (1, 2);
    let mut s = 0;
    while fib.0 <= 4000000 {
        if fib.0 % 2 == 0 {
            s += fib.0;
        }
        (fib.0, fib.1) = (fib.1, fib.0 + fib.1);
    }
    println!("{s}");
}
