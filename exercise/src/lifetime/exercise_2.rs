#[derive(Debug)]
#[allow(dead_code)]
struct Config {
    a: String,
    b: String,
}

static mut CONFIG: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    let x = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });
    Some(Box::leak(x))
}

pub fn invoke() {
    unsafe {
        CONFIG = init();
        println!("{:?}", CONFIG)
    }
}
