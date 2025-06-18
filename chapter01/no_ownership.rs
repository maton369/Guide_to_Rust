fn main() {
    let x = 5u32;
    print_number(x);
    println!("x is still available: {}", x); // xはまだ有効
}

fn print_number(n: u32) {
    println!("The number is: {}", n);
} // nがドロップされる
