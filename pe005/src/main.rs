fn main() {
    method_1();
}

fn method_1() {
    let mut i = 1;
    'main_loop: loop {
        for j in 2..=20 {
            if i % j != 0 {
                i += 1;
                continue 'main_loop;
            }
        }
        break;
    }
    println!("{i}");
}
