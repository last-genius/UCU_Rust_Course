use std::thread;
use std::sync::mpsc::channel;

let (tx, rx) = channel();

for i in 0..10 {
    let tx = tx.clone();
    thread::spawn(move|| {
        tx.send(i).unwrap();
    });
}

for _ in 0..10 {
    let j = rx.recv().unwrap();
    println!("{}", j);
}
