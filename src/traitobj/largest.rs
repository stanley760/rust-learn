fn largest<T:PartialOrd + Clone> (list: &[T]) -> T {
    let mut largetst = list[0].clone();
    for item in list.iter().clone() {
        if *item > largetst {
            largetst = item.clone();
        }
    }
    largetst
}

fn largestst<T:PartialOrd + Copy> (list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largetst<T:PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn invoke() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largestst(&number_list);
    println!("largestst :{}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largetst(&char_list);
    println!("largestst :{}", result);
    let result = largest(&number_list);
    println!("largest :{}", result);
}