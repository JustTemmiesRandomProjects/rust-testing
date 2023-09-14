fn main() {
    let mut x: f64 = 1.0;
    const inf: f64 = 2.0;
    loop {
        x = x * 1.1;
        println!("{x}");
        if x == inf {
            break;
        }
    }
}
