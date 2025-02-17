use std::sync::{Arc, Mutex};

fn arc_mutext_is_send_sync() {
    let a= Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();
    let handle = std::thread::spawn(move || {
        let mut data = b.lock().unwrap();
        *data += 1;
    });

    {
        let mut data = c.lock().unwrap();
        *data += 1;
    }

    handle.join().unwrap();
    println!("{:?}", a);
}

#[cfg(test)]
mod test {
    use crate::traitobj::send_sync_trait::arc_mutext_is_send_sync;
    #[test]
    fn test_arc_mutext_is_send_sync() {
        arc_mutext_is_send_sync();
    }
}