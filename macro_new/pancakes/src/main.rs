use hello_macro_derive::HelloMacro;
use hello_macro::HelloMacro;
fn main() {
    PancakesCr::hello_macro();
}

#[derive(HelloMacro)]
struct PancakesCr;