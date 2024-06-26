pub fn push_str() {
    let mut s = String::from("foo");
    s.push_str(" bar");
    println!("s is {}", &s);
    s.push('s');
    println!("now s is {}", &s);

    let s1 = String::from("halo ");
    let s2 = String::from("rust");

    println!("{}", s1 + &s2); // 解引用强制转换（deref coercion）

    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!();

    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }
    println!()
}