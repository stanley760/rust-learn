use core::{fmt, str};
use std::ops::Deref;


#[allow(dead_code)]
const MINI_STRING_MAX_LENGTH: usize = 30; 

#[allow(dead_code)]
struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LENGTH],
}

#[allow(dead_code)]
impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bt = v.as_ref().as_bytes();
        let len = bt.len();
        let mut data = [0u8; MINI_STRING_MAX_LENGTH];
        data[..len].copy_from_slice(bt);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;
    
    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
    
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",  self.deref())
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}

impl From<&str> for MyString {
    fn from(value: &str) -> Self {
        match value.len() > MINI_STRING_MAX_LENGTH  {
            true => Self::Standard(value.to_owned()),
            _ => Self::Inline(MiniString::new(value)),
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[cfg(test)]
mod test {
    use super::{MiniString, MyString};


    #[test]
    fn test_my_string() {
        let len1 = std::mem::size_of::<MyString>();
        let len2 = std::mem::size_of::<MiniString>();
        println!("length: MyString:{}, MiniString:{}", len1, len2);

        let s1: MyString = "hello world".into();
        let s2: MyString = "粉红墙上画凤凰凤凰画在粉红墙红凤凰粉凤凰红粉凤凰花凤凰skrrrrr".into();

        println!(
            "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
            s1,
            s1.len(),
            s1.chars().count(),
            s2,
            s2.len(),
            s2.chars().count()
        );
    
        // MyString 可以使用一切 &str 接口，感谢 Rust 的自动 Deref
        assert!(s1.ends_with("world"));
        assert!(s2.starts_with("粉"));
    }
}
