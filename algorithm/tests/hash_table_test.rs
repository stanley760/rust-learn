mod test {

    use algorithm::ArrayHashTable;

    #[test]
    fn test_hash_table() {
        let mut hash_table = ArrayHashTable::new();
        hash_table.put(1, "a");
        hash_table.put(2, "b");
        hash_table.put(3, "c");
        hash_table.print();
        assert_eq!(hash_table.get(1), Some(&"a".to_string()));
        assert_eq!(hash_table.get(2), Some(&"b".to_string()));
        assert_eq!(hash_table.get(3), Some(&"c".to_string()));
        assert_eq!(hash_table.get(4), None);
        hash_table.remove(1);
        assert_eq!(hash_table.get(1), None);
        assert_eq!(hash_table.entry_set().len(), 2);
        assert_eq!(hash_table.value_set().len(), 2);
        assert_eq!(hash_table.value_set(), vec!["b", "c"]);
    }
}
