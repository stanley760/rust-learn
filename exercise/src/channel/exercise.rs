use std::{sync::mpsc, thread, vec};

pub fn invoke() {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        vals.iter().for_each(|val| {
            tx.send(val.to_string()).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        });
    });

    rx.iter()
        .for_each(|rec| println!("receive the info: {}", rec));
}
