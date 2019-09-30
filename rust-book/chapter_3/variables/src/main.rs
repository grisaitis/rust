fn main() {
    let x = 0.5_f64;
    print_variable(x);
    let x = x * 2.0;
    print_variable(x);
    // overflow_wrap_around_demo();
    shadow_scope_demo();
}

fn print_variable(v: f64) {
    println!("{}", v);
}

fn shadow_scope_demo() {
    let y: u8 = 0;
    loop {
        // y never increments
        // this let statement is in a new scope
        let y = y + 1;
        println!("{}", y);
    }
}

fn overflow_wrap_around_demo() {
    let mut y: u8 = 0;
    loop {
        // this y does increment
        y += 1;
        println!("{}", y);
    }
}
