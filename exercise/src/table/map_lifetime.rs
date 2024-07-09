use std::collections::HashMap;
struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn invoke() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);

    let mut table = HashMap::new();
    table.insert("one", 1);

    let mut table1 = HashMap::new();
    table1.insert("two", 2);
    table1.insert("three", 3);
    table.extend(table1);
    for (key, value) in &table {
        println!("{}: {}", key, value);
    }
    let teams = vec![
        ("qing".to_string(), 12000),
        ("anglo".to_string(), 11000),
        ("derogatory".to_string(), 1000),
    ];
    for x in &teams {
        table.insert(&x.0, x.1);
    }
    println!("{:?}", table);

    let x1: HashMap<String, i32> = teams.into_iter().collect();
    println!("{:?}", x1);

    let x = 5;
    let r = &x;

    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
