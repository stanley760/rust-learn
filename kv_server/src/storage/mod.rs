mod memory;
pub use memory::MemTable;
use crate::error::kv::KvError;
use crate::{Kvpair, Value};
/// Storage trait function:
/// 
/// Stroage abstractly, we don't force on the data where to store, 
/// but need to define how store from the outside. 
pub trait Storage {
    // get a value contains the key in the table.
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    
    // set a value contains the key in the table, and return the old value.
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    
    // check the key is existed or not.
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    // remove a value contains the key in the table, and return the old value.
    fn remove(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    
    // get all of kv pairs from the table.
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    
    // get the iterator of kv pairs by loop the table.
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();

        test_relationship_functions(store);
    }
    
    #[test]
    fn memtable_get_all_should_work() {
        let store = MemTable::new();
        test_get_all(store);
    }

    #[test]
    fn memtable_iter_should_work() {
        let store = MemTable::new();
        test_get_iter(store);
    }
    
    fn test_relationship_functions(store: impl Storage) {
        
        let v = store.set("t1", "k1".into(), "v1".into());
        assert!(v.unwrap().is_none());
        let v1 = store.set("t1", "k1".into(), "v2".into());
        assert_eq!(v1.unwrap(), Some("v1".into()));
        
        let v = store.get("t1", "k1");
        assert_eq!(v.unwrap(), Some("v2".into()));
        
        assert_eq!(Ok(None), store.get("t1", "k2"));
        assert!(store.get("t2", "k1").unwrap().is_none());
        
        assert!(store.contains("t1", "k1").unwrap());
        assert!(!store.contains("t1", "k2").unwrap());
        assert!(!store.contains("t2", "k1").unwrap());
        
        let v = store.remove("t1", "k1");
        assert_eq!(v.unwrap(), Some("v2".into()));
        
        assert_eq!(Ok(None), store.remove("t1", "k2"));
        assert_eq!(Ok(None), store.remove("t2", "k1"));
    }
    
    fn test_get_all(store: impl Storage) {
        let _ = store.set("table2", "k1".into(), "v1".into());
        let _ = store.set("table2", "k2".into(), "v2".into());
        
        let mut data = store.get_all("table2").unwrap();
        
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        assert_eq!(data, vec![
            Kvpair::new("k1", "v1".into()), 
            Kvpair::new("k2", "v2".into())]
        );
    }

    fn test_get_iter(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();
        let mut data: Vec<_> = store.get_iter("t2").unwrap().collect();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())
            ]
        )
    }
}