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