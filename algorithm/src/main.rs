use crate::fibonacci::Fibonacci;

mod fibonacci;

fn main() {
    let mut li = Fibonacci::new();
    let option = li.at(10);
    println!("{}", option)
}
