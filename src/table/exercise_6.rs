use std::hash::BuildHasherDefault;
use std::collections::HashMap;
// 引入第三方的哈希函数
use twox_hash::XxHash64;

type FashHashMap<K, V> = HashMap<K, V, BuildHasherDefault<XxHash64>>;

pub fn invoke() {
    let mut hash: FashHashMap<i32, &'static str> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));
}
