struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher { query, value: None }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn invoke() {
    let mut c = Cacher::new(|a| a);
    let _v1 = c.value("rust");
    let v2 = c.value("golang");
    println!("v2:{}", v2);
}

#[test]
fn call_different_types() {
    let mut c = Cacher::new(|a| a);
    let _v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 1)
}
