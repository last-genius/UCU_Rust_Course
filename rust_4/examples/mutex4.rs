use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let mut vec = Vec::new();
    let mut a = Arc::new(Mutex::new(42));

    for _ in 1..6 {
        let counter = Arc::clone(&a);
        vec.push(thread::spawn(move || {
            for _ in 1..10 {
                let mut data = counter.lock().unwrap();
                *data += 1;
            }
        }));
    }

    for thread in vec {
        thread.join().unwrap();
    }
}
