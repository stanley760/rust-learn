use std::{sync::mpsc, thread};
use anyhow::Result;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("hello")?;
        Ok::<(), anyhow::Error>(())
    });
    let msg = rx.recv()?;
    println!("{}", msg);
    
    Ok(())
}