// defined mostly used data-struct
mod int_set;
mod list;
mod linked_list;
mod zip_list;
pub use linked_list::*;

use std::{cell::RefCell, rc::Rc, time::SystemTime};
use list::List;
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ObjectType {
    String, 
    List,
    Set,
    Zset,
    Hash,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ObjectEncoding {
    Raw,
    Int,
    Ht,
    ZipMap,
    LinkedList,
    ZipList,
    IntSet,
    SkipList,
    EmbStr,
}

pub trait ObjectData {
    fn bytes_ref(&self) -> &[u8] { panic!("This is not a byte slice") }
    fn sds_ref(&self) -> &str { panic!("This is not an Sds string") }
    fn raw_bytes(&self) -> &[u8] { panic!("This type has no raw bytes") }
    fn integer(&self) -> i64 { panic!("This is not an integer") }
    fn linked_list_ref(&self) -> &List { panic!("This is not a List") }
    fn linked_list_mut(&mut self) -> &mut List { panic!("This is not a List") }
    // fn set_ref(&self) -> &Set { panic!("This is not a Set") }
    // fn set_mut(&mut self) -> &mut Set { panic!("This is not a Set") }
    // fn zip_list_ref(&self) -> &ZipList { panic!("This is not a ZipList") }
    // fn zip_list_mut(&mut self) -> &mut ZipList { panic!("This is not a ZipList") }
    // fn hash_table_ref(&self) -> &Dict<RobjPtr, RobjPtr> { panic!("This is not a hash table") }
    // fn int_set_ref(&self) -> &IntSet { panic!("This is not an IntSet") }
    // fn int_set_mut(&mut self) -> &mut IntSet { panic!("This is not an IntSet") }
    // fn set_wrapper_ref(&self) -> &dyn SetWrapper { panic!("This is not as SetWrapper") }
    // fn set_wrapper_mut(&mut self) -> &mut dyn SetWrapper { panic!("This is not as SetWrapper") }
    // fn zset_ref(&self) -> &Zset { panic!("This is not a Zset") }
    fn encoding(&self) -> ObjectEncoding;

}


type Pointer = Box<dyn ObjectData>;
pub type RedisobjPtr = Rc<RefCell<RedisObject>>;


pub struct RedisObject {
    obj_type: ObjectType,
    encoding: ObjectEncoding,
    lru: SystemTime,
    ptr: Pointer,
}