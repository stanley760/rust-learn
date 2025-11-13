use algorithm::HashTableChaining;

#[test]
pub fn test_hash_table_chaining() {
    let mut hash_table = HashTableChaining::new();
    hash_table.put(1, "a".to_string());
    hash_table.put(2, "b".to_string());
    hash_table.put(3, "c".to_string());
    hash_table.put(4, "d".to_string());
    hash_table.put(5, "e".to_string());

    hash_table.print();
}
