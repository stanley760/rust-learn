use anyhow::{anyhow, Result};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Deref, Mul},
};

pub struct Vector<T> {
    data: Vec<T>,
}
impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Vector { data: data.into() }
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default + Debug,
{
    if a.len() != b.len() {
        return Err(anyhow!("Matrix dot error: a.len() != b.len()"));
    }

    let mut res = T::default();
    for i in 0..a.len() {
        res += a[i] * b[i];
    }
    Ok(res)
}
