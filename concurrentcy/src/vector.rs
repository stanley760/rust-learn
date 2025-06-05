use anyhow::{Result, anyhow};
use std::{fmt::Debug, ops::{Add, AddAssign, Mul}};



pub struct Vector<T> {
    data: Vec<T>,
}

fn dot_product<T>(a: Vec<T>, b: Vec<T>) -> Result<T>
    where T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default + Debug {
    if a.len() != b.len() {
        return Err(anyhow!("Matrix dot error: a.len() != b.len()"));
    }

    let mut res = T::default();
    for i in 0..a.len() {
        res += a[i] * b[i];
    }
    Ok(res)
}