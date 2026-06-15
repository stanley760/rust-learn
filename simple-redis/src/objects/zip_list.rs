#[derive(Clone)]
enum Encoding {
    Str(usize),
    Int(i64),
}

impl Encoding {
    fn unwrap_str(&self) -> usize {
        match self {
            Self::Str(s) => *s,
            _ => panic!("this is not in encoding with the string")
        }
    }

    fn unwrap_int(&self) -> i64 {
        match self {
            Self::Int(v) => *v,
            _ => panic!("this is not in encoding with a int"),
        }
    }

    fn is_str(&self) -> bool {
        match self {
            Self::Str(_) => true,
            _ => false,
        }
    }

    fn is_int(&self) -> bool {
        !self.is_str()
    }

    

}