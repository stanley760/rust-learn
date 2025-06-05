use std::fmt::Debug; 
use std::ops::{Add, AddAssign, Mul};
use anyhow::{Result, anyhow};


#[derive(Debug)]
pub struct Matrix<T> {
    pub data: Vec<T>,
    pub row: usize,
    pub col: usize,
}


pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>> 
    where T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default + Debug {
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: a.col != b.row"));
    }
    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }
    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

impl<T> Matrix<T> 
    where T: Debug {
        
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Matrix { data: data.into(), row, col }
    }
}