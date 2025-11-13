#[allow(dead_code)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Elixir => "Elixir",
            Language::Haskell => "Haskell",
        }
    }
}
#[allow(dead_code)]
fn print_ref(s: impl AsRef<str>) {
    println!("as_ref: {}", s.as_ref());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_as_ref_trait() {
        let lang = Language::Rust;
        // &str 实现了 AsRef<str>
        print_ref("Hello world!");
        // String 实现了 AsRef<str>
        print_ref("Hello world!".to_string());
        // 我们自己定义的 enum 也实现了 AsRef<str>
        print_ref(lang);
    }
}
