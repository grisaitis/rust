fn main() {
    let tup: (u8, f64, bool) = (100, 3.14, false);
    println!("{}", tup.1);

    let (x, y, z) = tup;
    println!("{}", y);

    let a = [10_u8; 10];
    println!("{}", a[2]);

    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];
}
