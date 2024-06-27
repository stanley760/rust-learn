pub fn invoke() {
    let vec1 = vec![0; 3];
    let vec2 = Vec::from([0, 0, 0]);
    assert_eq!(vec1, vec2);
    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]); // 附加数据到 v
    println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());

    v.reserve(100); // 调整 v 的容量，至少要有 100 的容量
    println!(
        "Vector（reserve） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );

    v.shrink_to_fit(); // 释放剩余的容量，一般情况下，不会主动去释放容量
    println!(
        "Vector（shrink_to_fit） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );

    assert!(!v.is_empty());
    v.insert(2, 4);
    println!("v：{:?}", v);
    assert_eq!(v.remove(2), 4);
    assert_eq!(v.pop(), Some(3));
    assert_eq!(v.pop(), Some(2));
    assert_eq!(v.pop(), Some(1));
    assert_eq!(v.pop(), None);
    v.clear();
    let mut vec3 = [111, 333].to_vec();
    v.append(&mut vec3); // ************** append导致vec3清空数据  ****************
                         //  self.append_elements(other.as_slice() as _);
                         //  other.set_len(0);
    v.truncate(1); // 截断到指定长度

    v.retain(|x| *x > 10); // 保留满足条件的元素

    println!("v: {:?}", v);

    let mut vec4 = vec![111, 222, 333, 444, 555, 666];
    let mut m: Vec<_> = vec4.drain(1..=3).collect(); //删除指定范围的元素，同时获取被删除元素的迭代器
    let vec5 = m.split_off(1);
    println!("vec4: {:?}", vec4);
    println!("vec5: {:?}", vec5)
}
