use std::fmt::{self, Display, Formatter};
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
struct Developer {
    name: String,
    age: u8,
    language: Language,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}
#[allow(dead_code)]
impl Developer {

    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl Display for Developer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}, age: {}, language: {:?}", self.name, self.age, self.language)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_default_display_trait() {
        // 使用 T::default()
        let dev1 = Developer::default();
        // 使用 Default::default()，但此时类型无法通过上下文推断，需要提供类型
        let dev2: Developer = Default::default();
        // 使用 T::new
        let dev3 = Developer::new("Tyr");
        println!("dev1: {}\\ndev2: {}\\ndev3: {:?}", dev1, dev2, dev3);
    }
}