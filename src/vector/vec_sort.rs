pub fn invoke() {
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort_unstable();
    println!("{:?}", vec);
    let mut vec1 = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    vec1.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", vec1);

    let mut vec2 = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("Al".to_string(), 30),
        Person::new("John".to_string(), 1),
        Person::new("John".to_string(), 25),
        Person::new("Alice".to_string(), 25),
        Person::new("Bob".to_string(), 10),
        Person::new("Charlie".to_string(), 35),
    ];
    vec2.sort_unstable();
    // vec2.sort_unstable_by(|a, b| a.age.cmp(&b.age));
    println!("{:?}", vec2);
}


#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}