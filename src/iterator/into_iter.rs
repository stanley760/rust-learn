use std::collections::HashMap;

/// into_iter 会夺走所有权
/// iter 是借用
/// iter_mut 是可变借用
pub fn invoke() {
    let values = vec![1, 2, 3];
    for v in values.into_iter() {
        println!("{}", v);
    }
    // println!("{:?}", values); // errors: values 的所有权在上面 `for` 循环中已经被转移走
    let values1 = vec![1, 2, 3];
    let iter = values1.iter();
    println!("{:?}", values1);   // values1 仍然可用
    let total:i32 = iter.sum();  // 消费者适配器
    assert_eq!(total, 6);
    // println!("{:?}", iter);  // errors: sum 拿走了该迭代器的所有权
    let mut values2 = vec![1, 2, 3];

    let mut iter = values2.iter_mut();

    if let Some(v) = iter.next() {
        *v = 0;
    }
    println!("{:?}", values2);

    let va : Vec<_> = values2.iter().map(|x| x + 1).collect();
    assert_eq!(va, vec![1, 3, 4]);
    
    let names = ["LiHua", "LiLei", "HanMeiMei"];
    let ages = [18, 19, 20];
    // zip 是一个迭代器适配器，它的作用就是将两个迭代器的内容压缩到一起，
    // 形成 Iterator<Item=(ValueFromA, ValueFromB)> 这样的新的迭代器
    let folks: HashMap<_,_> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}", folks);
    let shoes = vec![
        Shoe { size: 10, style: "sneaker".to_string() },
        Shoe { size: 13, style: "sandal".to_string() },
        Shoe { size: 10, style: "boot".to_string() },
    ];
    let vec1 = shoe_in_size(shoes, 13u32);
    println!("vec:{:?}", vec1)
}

fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[derive(Debug)]
#[allow(dead_code)]
struct Shoe {
    size: u32,
    style: String,
}

#[test]
fn invoke_test() {
    invoke();
}