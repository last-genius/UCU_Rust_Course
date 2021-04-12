use std::thread;
use std::sync::mpsc::channel;

// MPSC Channel returns two ends:
// * tx - a transmitter, which can be cloned
// * rx - a receiver, which can't be cloned
let (tx, rx) = channel();

thread::spawn(move|| {
    tx.send(10).unwrap();
});

println!("{}", rx.recv().unwrap());
