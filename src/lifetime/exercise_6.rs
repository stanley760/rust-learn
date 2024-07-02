use std::fmt::Display;

fn print_a<T: Display + 'static>(t: &T) {
    println!("a: {}", t);
}

fn print_b<T>(t: &T)
    where T: Display + 'static {
    println!("b: {}", t)
}

fn print_c(t: &'static dyn Display) {
    println!("c: {}", t);
}

fn print_d(t: &'static impl Display) {
    println!("d: {}", t);
}

fn print_e(t: &(dyn Display + 'static)) {
    println!("e: {}", t)
}

fn print_f(t: &(impl Display + 'static)) {
    println!("f: {}", t);
}

fn print_g(t: &'static String) {
    println!("g: {}", t);
}

pub fn invoke() {
    let mut string = "First".to_owned();

    string.push_str(string.to_uppercase().as_str());
    print_a(&string);
    print_b(&string);
    print_c(Box::leak(Box::new(string.clone()))); // Compilation error
    print_d(Box::leak(Box::new(string.clone()))); // Compilation error
    print_e(&string);
    print_f(&string);
    print_g(Box::leak(Box::new(string))); // Compilation error
}