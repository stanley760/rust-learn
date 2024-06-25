pub fn invoke() {
    let mut v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i)
    }
    v.push(1);
    println!("{:?}", v);

    let mut v1 = vec![1, 2, 3, 4, 5];
    // let first =  &v1[0];// error: 放在push之前执行报错,不可变借用发生在此处
    let first = v1[0];
    v1.push(6);
    println!("The first element is: {first}");
}