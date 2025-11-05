use std::borrow::Cow;
use serde::Deserialize;
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct User<'input> {
    #[serde(borrow)]
    name: Cow<'input, str>,
    age: u8,
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use super::User;


    #[test]
    fn test_use_cow() {
        let input = r#"{ "name": "Tyr", "age": 18 }"#;
        let user: User = serde_json::from_str(input).unwrap();

        match user.name {
            Cow::Borrowed(x) => println!("borrowed {}", x),
            Cow::Owned(x) => println!("owned {}", x),
        }
        println!("age: {}", user.age);
    }
}