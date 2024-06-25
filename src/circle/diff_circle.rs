pub fn invoke() {
    let mut i : i32 = 1;
    let sum  = loop {
        i += 1;
        if i == 100 {
           break i;
        }
    };
    println!("{:?}", sum);


    let mut i : i32 = 1;

    while i != 100 {
        i += 1;
    }
    println!("{:?}", i);
    let mut numb = 1;
    let sum = |x: &mut i32| *x += 1;
    for _ in 1.. 100 {
        sum(&mut numb);
    }
    println!("{}", numb);
}