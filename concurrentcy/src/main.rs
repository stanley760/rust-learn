use anyhow::Result;
use concurrentcy::{multiply, Matrix};


fn main() -> Result<()> {
    let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
    let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
    let c = multiply(&a, & b)?;
    println!("{:?}", c);
    Ok(())
}