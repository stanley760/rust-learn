use algorithm::MyList;

#[test]
#[allow(non_snake_case)]
pub fn test_MyList() {
    let mut list = MyList::new(10);
    list.add(1);
    list.add(2);
    list.add(3);
    list.insert(3, 4);
    assert_eq!(list.to_array(), vec![1, 2, 3, 4]);
    list.remove(3);
    assert_eq!(list.to_array(), vec![1, 2, 3]);
    list.set(2, 5);
    assert_eq!(list.to_array(), vec![1, 2, 5]);
    let val = list.get(2);
    assert_eq!(val, 5);
}
