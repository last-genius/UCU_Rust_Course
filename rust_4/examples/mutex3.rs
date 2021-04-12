use std::{rc::Rc, sync::Mutex, thread};

fn main() {
    let mut vec = Vec::new();
    let mut a = Rc::new(Mutex::new(42));

    for _ in 1..6 {
        vec.push(thread::spawn(move || {
            for _ in 1..10 {
                let mut data = a.lock().unwrap();
                *data += 1;
            }
        }));
    }

    for thread in vec {
        thread.join().unwrap();
    }
}
