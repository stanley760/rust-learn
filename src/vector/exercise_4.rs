pub fn invoke() {
    let mut v = Vec::from([1, 2, 3]);
    for i in v.iter() {
        println!("{:?}", v[*i - 1])
    }

    for i in 0..5 {
        match v.get(i) {
            Some(x) => v[i] = x + 1,
            None => v.push(i + 2),
        }
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!")
}
