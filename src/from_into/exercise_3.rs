use std::fs;
use std::io;
use std::num;

#[derive(Debug)]
#[allow(dead_code)]
enum CliErr {
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
}

impl From<io::Error> for CliErr {
    fn from(err: io::Error) -> Self {
        CliErr::IoError(err)
    }
}

impl From<num::ParseIntError> for CliErr {
    fn from(value: num::ParseIntError) -> Self {
        CliErr::ParseIntError(value)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliErr> {
    // ? 自动将 io::Error 转换成 CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

pub fn invoke() {
    println!("{:#?}", open_and_parse_file("num.txt"))
}