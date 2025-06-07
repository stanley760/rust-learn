use std::fmt::Debug; 
use std::ops::{Add, AddAssign, Mul};
use anyhow::{Result, anyhow};

use crate::{dot_product, Vector};


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
            let row_a = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col).map(|&x| x).collect::<Vec<T>>();
            let col_b = Vector::new(col_data);
            data[i * b.col + j] += dot_product(row_a, col_b)?;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix_multiply() {
        let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let b = Matrix::new(vec![5, 6, 7, 8], 2, 2);
        let c = multiply(&a, &b).unwrap();
        assert_eq!(c.data, vec![19, 22, 43, 50]);
        
    }
}