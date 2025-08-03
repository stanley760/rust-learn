use std::fmt::Debug; 
use std::ops::{Add, AddAssign, Mul};
use std::sync::mpsc;
use std::thread;
use anyhow::{anyhow, Ok, Result};

use crate::{dot_product, Vector};

const THREAD_NUM: usize = 4;
#[derive(Debug)]
pub struct Matrix<T> {
    pub data: Vec<T>,
    pub row: usize,
    pub col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    data: T,
}

pub struct Msg<T> {
    pub input: MsgInput<T>,
    pub output: oneshot::Sender<MsgOutput<T>>,
}


pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>> 
    where T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default + Debug + Send + 'static {
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: a.col != b.row"));
    }

    let sender = (0..THREAD_NUM)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.output.send(MsgOutput {
                        idx: msg.input.idx,
                        data: value,
                    }) {
                        eprintln!("Error sending message: {:?}", e);
                    }
                }
                Ok(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let mut receiver = Vec::with_capacity(a.row * b.col);
    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            let row_a = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col).map(|&x| x).collect::<Vec<T>>();
            let col_b = Vector::new(col_data);
            
            let idx = i * b.col + j;
            let msg_input = MsgInput::new(idx, row_a, col_b);
            let (tx, rx) = oneshot::channel::<MsgOutput<T>>();
            let msg = Msg::new(msg_input, tx);
            let thread_idx = idx % THREAD_NUM;
            if let Err(e) = sender[thread_idx].send(msg) {
                eprintln!("Error sending message to thread {}: {:?}", thread_idx, e);
            }
            receiver.push(rx);
        }
    }

    for rx in receiver {
        let output = rx.recv()
            .map_err(|e| anyhow!("Error receiving message: {:?}", e))?;
        data[output.idx] = output.data;
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

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        MsgInput { idx, row, col }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, output: oneshot::Sender<MsgOutput<T>>) -> Self {
        Msg { input, output }
    }
}

impl<T> Mul for Matrix<T> 
    where T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default + Debug + Send + 'static {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("matrix multiplication failed")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix_multiply() {
        let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let b = Matrix::new(vec![5, 6, 7, 8], 2, 2);
        let c = a * b;
        assert_eq!(c.data, vec![19, 22, 43, 50]);
        
    }
}