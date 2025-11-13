use std::{sync::mpsc, thread};

pub fn invoke() {
    let (tx, rx) = mpsc::channel::<String>();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = [
            String::from("clone: hi"),
            String::from("clone: from"),
            String::from("clone: the"),
            String::from("clone: thread"),
        ];
        vals.iter().for_each(|val| {
            tx1.send(val.to_string()).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        });
    });

    thread::spawn(move || {
        let vals = [
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
