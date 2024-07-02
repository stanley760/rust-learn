/// into_iter 会夺走所有权
/// 
/// 
pub fn invoke() {
    let values = vec![1, 2, 3];
    for v in values.into_iter() {
        println!("{}", v);
    }
    
}