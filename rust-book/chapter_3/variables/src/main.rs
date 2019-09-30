fn main() {
    let x = 0.5_f64;
    print_variable(x);
    let x = x * 2.0;
    print_variable(x);
}

fn print_variable(v: f64) {
    println!("{}", v);
}
