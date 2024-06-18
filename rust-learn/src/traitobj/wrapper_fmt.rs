use std::fmt;
use std::fmt::Formatter;

struct Wrapper(Vec<&'static str>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}