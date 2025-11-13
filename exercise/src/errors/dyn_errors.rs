use std::{error::Error, fmt::Display};

#[derive(Debug)]
#[allow(dead_code)]
struct CusError;
#[allow(clippy::recursive_format_impl)]
impl Display for CusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for CusError {}

fn foo(num: u32) -> Result<String, Box<dyn Error>> {
    match num {
        1 => Ok("one".to_string()),
        _ => {
            let err = CusError;
            Err(Box::new(err))
        }
    }
}

pub fn test_dyn_error() {
    let result = foo(2);
    match result {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("err:{}", e),
    }
}
