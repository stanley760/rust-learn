use std::{
    collections::LinkedList,
    ops::{Deref, DerefMut, Index},
};
struct List<T>(LinkedList<T>);

impl<T> Deref for List<T> {
    type Target = LinkedList<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Index<isize> for List<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        let len = self.0.len();

        let n = (index as usize).checked_rem_euclid(len).unwrap();
        &self.0.iter().nth(n).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn it_works() {
        let mut list: List<u32> = List::default();
        for i in 0..16 {
            list.push_back(i);
        }
    
        assert_eq!(list[0], 0);
        assert_eq!(list[5], 5);
        assert_eq!(list[15], 15);
        assert_eq!(list[16], 0);
        assert_eq!(list[-1], 15);
        assert_eq!(list[128], 0);
        assert_eq!(list[-128], 0);
    }
}

