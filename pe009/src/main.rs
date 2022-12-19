fn pe009() -> i32 {
    for i in 1..1000 {
        for j in i+1..1000 {
            for k in j+1..1000 {
                if i + j + k == 1000 {
                    if i*i + j*j == k*k {
                        return i*j*k;
                    }
                }
            }
        }
    }
    return 0;
}

fn main() {
    let answer = pe009();
    println!("{answer}");
}
