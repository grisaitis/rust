fn main() {
    let tup: (u8, f64, bool) = (100, 3.14, false);
    println!("{}", tup.1);

    let (x, y, z) = tup;
    println!("{}", y);
}
